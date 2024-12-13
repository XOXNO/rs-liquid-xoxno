multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::contexts::base::StorageCache;
use crate::errors::*;

use super::config;

pub const UNDELEGATE_TOKEN_URI: &[u8] =
    b"https://ipfs.io/ipfs/QmY4jtQh6M24uAFR3LcyV7QmL8pkL6zFxXyPXBuzo5sdX5";

#[type_abi]
#[derive(TopEncode, TopDecode, PartialEq, Eq, Copy, Clone, Debug)]
pub enum State {
    Inactive,
    Active,
}

#[multiversx_sc::module]
pub trait LiquidityPoolModule:
    config::ConfigModule + multiversx_sc_modules::default_issue_callbacks::DefaultIssueCallbacksModule
{
    fn pool_add_liquidity(
        &self,
        token_amount: &BigUint,
        storage_cache: &mut StorageCache<Self>,
    ) -> BigUint {
        let ls_amount = self.get_ls_token_amount(token_amount, &storage_cache);

        storage_cache.ls_token_supply += &ls_amount;
        storage_cache.virtual_xoxno_reserve += token_amount;

        ls_amount
    }

    fn pool_remove_liquidity(
        &self,
        token_amount: &BigUint,
        storage_cache: &mut StorageCache<Self>,
    ) -> BigUint {
        let xoxno_amount = self.get_xoxno_amount(token_amount, storage_cache);
        storage_cache.ls_token_supply -= token_amount;
        storage_cache.virtual_xoxno_reserve -= &xoxno_amount;

        xoxno_amount
    }

    fn get_xoxno_amount(
        &self,
        ls_token_amount: &BigUint,
        storage_cache: &StorageCache<Self>,
    ) -> BigUint {
        require!(
            &storage_cache.ls_token_supply >= ls_token_amount,
            ERROR_NOT_ENOUGH_LP
        );

        let xoxno_amount =
            ls_token_amount * &storage_cache.virtual_xoxno_reserve / &storage_cache.ls_token_supply;
        require!(xoxno_amount > 0u64, ERROR_INSUFFICIENT_LIQ_BURNED);

        xoxno_amount
    }

    fn get_ls_token_amount(
        &self,
        token_amount: &BigUint,
        storage_cache: &StorageCache<Self>,
    ) -> BigUint {
        // Ensure the provided token_amount is greater than 0
        require!(
            token_amount > &BigUint::from(0u64),
            ERROR_INSUFFICIENT_LIQ_BURNED
        );

        // Calculate the ls_token_amount based on the provided token_amount and including rewards
        let ls_token_amount = if storage_cache.virtual_xoxno_reserve > 0 {
            token_amount.clone() * &storage_cache.ls_token_supply
                / &storage_cache.virtual_xoxno_reserve
        } else {
            token_amount.clone()
        };

        // Ensure that the calculated ls_token_amount is greater than zero
        require!(ls_token_amount > 0, ERROR_INSUFFICIENT_LIQUIDITY);

        ls_token_amount
    }

    fn mint_ls_token(&self, amount: BigUint) -> EsdtTokenPayment<Self::Api> {
        self.ls_token().mint(amount)
    }

    fn burn_ls_token(&self, amount: &BigUint) {
        self.ls_token().burn(amount);
    }

    fn mint_unstake_tokens<T: TopEncode>(&self, attributes: &T) -> EsdtTokenPayment<Self::Api> {
        let token_map = self.unstake_token();
        let nft = token_map.nft_create(BigUint::from(1u64), attributes);

        let uri = ManagedBuffer::from(UNDELEGATE_TOKEN_URI);
        self.send()
            .nft_add_uri(token_map.get_token_id_ref(), nft.token_nonce, uri);
        nft
    }

    fn burn_unstake_tokens(&self, token_nonce: u64) {
        self.unstake_token()
            .nft_burn(token_nonce, &BigUint::from(1u64));
    }
}
