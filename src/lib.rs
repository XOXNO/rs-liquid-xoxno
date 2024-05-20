#![no_std]

#[allow(unused_imports)]
use multiversx_sc::imports::*;
pub mod config;
pub mod contexts;
pub mod errors;
pub mod events;
pub mod liquidity_pool;
use crate::{
    config::{UnstakeTokenAttributes, UNBOND_PERIOD},
    errors::*,
};
use contexts::base::*;

#[multiversx_sc::contract]
pub trait RsLiquidXoxno:
    config::ConfigModule
    + liquidity_pool::LiquidityPoolModule
    + events::EventsModule
    + multiversx_sc_modules::ongoing_operation::OngoingOperationModule
    + multiversx_sc_modules::default_issue_callbacks::DefaultIssueCallbacksModule
{
    #[init]
    fn init(&self, main_token: &TokenIdentifier) {
        self.main_token().set(main_token);
    }

    #[upgrade]
    fn upgrade(&self) {}

    #[payable("*")]
    #[endpoint(delegate)]
    fn add_liquidity(&self) {
        let mut storage_cache = StorageCache::new(self);
        let caller = self.blockchain().get_caller();

        let staked_tokens = self.call_value().single_esdt();
        require!(
            staked_tokens.token_identifier == storage_cache.main_token_id,
            ERROR_WRONG_TOKEN
        );

        require!(
            self.is_state_active(storage_cache.contract_state),
            ERROR_NOT_ACTIVE
        );

        let ls_token_amount = self.pool_add_liquidity(&staked_tokens.amount, &mut storage_cache);
        let user_payment = self.mint_ls_token(ls_token_amount);
        self.send().direct_esdt(
            &caller,
            &user_payment.token_identifier,
            user_payment.token_nonce,
            &user_payment.amount,
        );

        self.emit_add_liquidity_event(&storage_cache, &caller, user_payment.amount);
    }

    #[payable("*")]
    #[endpoint(unDelegate)]
    fn remove_liquidity(&self) {
        let mut storage_cache = StorageCache::new(self);
        let caller = self.blockchain().get_caller();
        let payment = self.call_value().single_esdt();

        require!(
            self.is_state_active(storage_cache.contract_state),
            ERROR_NOT_ACTIVE
        );
        require!(
            storage_cache.ls_token_id.is_valid_esdt_identifier(),
            ERROR_LS_TOKEN_NOT_ISSUED
        );
        require!(
            payment.token_identifier == storage_cache.ls_token_id,
            ERROR_BAD_PAYMENT_TOKEN
        );
        require!(payment.amount > 0, ERROR_BAD_PAYMENT_AMOUNT);

        let xoxno_to_unstake = self.pool_remove_liquidity(&payment.amount, &mut storage_cache);

        self.burn_ls_token(&payment.amount);

        let current_epoch = self.blockchain().get_block_epoch();
        let unbond_epoch = current_epoch + UNBOND_PERIOD;

        self.unstake_token_supply()
            .update(|x| *x += &xoxno_to_unstake);

        let virtual_position = UnstakeTokenAttributes {
            unstake_epoch: current_epoch,
            unstake_amount: xoxno_to_unstake,
            unbond_epoch,
        };

        let user_payment = self.mint_unstake_tokens(&virtual_position);
        self.send().direct_esdt(
            &caller,
            &user_payment.token_identifier,
            user_payment.token_nonce,
            &user_payment.amount,
        );

        self.emit_remove_liquidity_event(&storage_cache, payment.amount, user_payment.amount);
    }

    #[payable("*")]
    #[endpoint(withdraw)]
    fn withdraw(&self) {
        self.blockchain().check_caller_is_user_account();
        let mut storage_cache = StorageCache::new(self);
        let caller = self.blockchain().get_caller();
        let payment = self.call_value().single_esdt();

        require!(
            self.is_state_active(storage_cache.contract_state),
            ERROR_NOT_ACTIVE
        );
        require!(
            payment.token_identifier == self.unstake_token().get_token_id(),
            ERROR_BAD_PAYMENT_TOKEN
        );
        require!(payment.amount > 0, ERROR_BAD_PAYMENT_AMOUNT);

        let unstake_token_attributes: UnstakeTokenAttributes<Self::Api> = self
            .unstake_token()
            .get_token_attributes(payment.token_nonce);

        let current_epoch = self.blockchain().get_block_epoch();
        require!(
            current_epoch >= unstake_token_attributes.unbond_epoch,
            ERROR_UNSTAKE_PERIOD_NOT_PASSED
        );

        let unstake_amount = unstake_token_attributes.unstake_amount;

        // Hnadle the case when the user tries to withdraw more than the total withdrawn amount (in case of the last user withdrawal)
        if unstake_amount > storage_cache.total_withdrawn_xoxno {
            storage_cache.total_withdrawn_xoxno = BigUint::from(0u64);
        } else {
            storage_cache.total_withdrawn_xoxno -= &unstake_amount;
        }

        let map_unstake = self.unstake_token_supply();
        let unstake_supply = map_unstake.get();
        // Handle the case when the user tries to withdraw more than the total supply of the unstake token (in case of the last user withdrawal)
        if unstake_amount > unstake_supply {
            map_unstake.set(&BigUint::from(0u64));
        } else {
            map_unstake.set(&unstake_supply - &unstake_amount);
        }
        self.burn_unstake_tokens(payment.token_nonce);
        self.send()
            .direct_esdt(&caller, &storage_cache.main_token_id, 0, &unstake_amount)
    }

    #[payable("*")]
    #[endpoint(addRewards)]
    fn add_rewards(&self) {
        let mut storage_cache = StorageCache::new(self);
        let caller = self.blockchain().get_caller();

        let staked_tokens = self.call_value().single_esdt();
        require!(
            staked_tokens.token_identifier == storage_cache.main_token_id,
            ERROR_WRONG_TOKEN
        );

        require!(
            self.is_state_active(storage_cache.contract_state),
            ERROR_NOT_ACTIVE
        );
        storage_cache.virtual_xoxno_reserve += staked_tokens.amount.clone();

        self.emit_add_rewards_event(&storage_cache, &caller, staked_tokens.amount);
    }

    #[view(getMainTokenAmountForPosition)]
    fn get_ls_value_for_position(&self, ls_token_amount: BigUint) -> BigUint {
        let storage_cache = StorageCache::new(self);
        self.get_xoxno_amount(&ls_token_amount, &storage_cache)
    }

    #[view(getLsTokenAmountForMainTokenAmount)]
    fn get_ls_amount_for_position(&self, main_token_amount: BigUint) -> BigUint {
        let storage_cache = StorageCache::new(self);
        self.get_ls_token_amount(&main_token_amount, &storage_cache)
    }
}
