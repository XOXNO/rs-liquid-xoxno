#![no_std]

#[allow(unused_imports)]
use multiversx_sc::imports::*;
pub mod config;
pub mod contexts;
pub mod errors;
pub mod events;
pub mod liquidity_pool;
pub mod rs_xoxno_proxy;

use crate::{
    config::{UnstakeTokenAttributes, INITIAL_EXCHANGE_RATE, UNBOND_PERIOD},
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
    + ContractBase
{
    #[init]
    fn init(&self, main_token: &TokenIdentifier) {
        self.main_token().set(main_token);
    }

    #[upgrade]
    fn upgrade(&self) {}

    #[payable("*")]
    #[endpoint(delegate)]
    fn delegate(&self, delegator: OptionalValue<ManagedAddress>) {
        let mut storage_cache = StorageCache::new(self);
        let user = match delegator {
            OptionalValue::Some(user) => user,
            OptionalValue::None => self.blockchain().get_caller(),
        };

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
        self.tx().to(&user).payment(&user_payment).transfer();

        self.emit_delegate_event(&storage_cache, &user, user_payment.amount, staked_tokens.amount);
    }

    #[payable("*")]
    #[endpoint(unDelegate)]
    fn un_delegate(&self) {
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

        storage_cache.total_unstaked_xoxno += &xoxno_to_unstake;

        let virtual_position = UnstakeTokenAttributes {
            unstake_epoch: current_epoch,
            original_amount: xoxno_to_unstake.clone(),
            share_amount: payment.amount.clone(),
            unbond_epoch,
        };
        let user_payment = self.mint_unstake_tokens(&virtual_position);
        self.tx().to(&caller).payment(&user_payment).transfer();
        self.emit_remove_liquidity_event(
            &storage_cache,
            user_payment,
            payment.amount,
            xoxno_to_unstake.clone(),
        );
    }

    #[payable("*")]
    #[endpoint(withdraw)]
    fn withdraw(&self) {
        let mut storage_cache = StorageCache::new(self);
        let caller = self.blockchain().get_caller();
        let payments = self.call_value().all_esdt_transfers().clone_value();
        self.unstake_token().require_all_same_token(&payments);

        require!(
            self.is_state_active(storage_cache.contract_state),
            ERROR_NOT_ACTIVE
        );
        let current_epoch = self.blockchain().get_block_epoch();
        let mut total_unstaked = BigUint::zero();
        for payment in payments.iter() {
            require!(payment.amount > 0, ERROR_BAD_PAYMENT_AMOUNT);

            let unstake_token_attributes: UnstakeTokenAttributes<Self::Api> = self
                .unstake_token()
                .get_token_attributes(payment.token_nonce);

            require!(
                current_epoch >= unstake_token_attributes.unbond_epoch,
                ERROR_UNSTAKE_PERIOD_NOT_PASSED
            );

            let unstake_amount = unstake_token_attributes.original_amount;

            // Hnadle the case when the user tries to withdraw more than the total withdrawn amount (in case of the last user withdrawal)
            if unstake_amount > storage_cache.total_unstaked_xoxno {
                storage_cache.total_unstaked_xoxno = BigUint::from(0u64);
            } else {
                storage_cache.total_unstaked_xoxno -= &unstake_amount;
            }

            total_unstaked += unstake_amount;
            self.burn_unstake_tokens(payment.token_nonce);
        }
        if total_unstaked > 0 {
            self.tx()
                .to(&caller)
                .single_esdt(&storage_cache.main_token_id, 0, &total_unstaked)
                .transfer();
        }
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

    #[view(getExchangeRate)]
    fn get_exchange_rate(&self) -> BigUint {
        let storage_cache = StorageCache::new(self);

        // The initial exchange rate between XOXNO and LXOXNO is fixed to one
        if storage_cache.ls_token_supply.clone() == BigUint::zero() {
            return BigUint::from(INITIAL_EXCHANGE_RATE);
        }

        storage_cache.virtual_xoxno_reserve.clone() * BigUint::from(INITIAL_EXCHANGE_RATE)
            / storage_cache.ls_token_supply.clone()
    }
}
