multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::liquidity_pool::State;

pub const UNBOND_PERIOD: u64 = 10;
pub const INITIAL_EXCHANGE_RATE: u64 = 1_000_000_000_000_000_000;

#[type_abi]
#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, Clone, PartialEq, Eq, Debug)]
pub struct UnstakeTokenAttributes<M: ManagedTypeApi> {
    pub original_amount: BigUint<M>,
    pub share_amount: BigUint<M>,
    pub unstake_epoch: u64,
    pub unbond_epoch: u64,
}

#[multiversx_sc::module]
pub trait ConfigModule:
    multiversx_sc_modules::default_issue_callbacks::DefaultIssueCallbacksModule
{
    #[only_owner]
    #[payable("EGLD")]
    #[endpoint(registerLsToken)]
    fn register_ls_token(
        &self,
        token_display_name: ManagedBuffer,
        token_ticker: ManagedBuffer,
        num_decimals: usize,
    ) {
        let payment_amount = self.call_value().egld_value().clone_value();
        self.ls_token().issue_and_set_all_roles(
            payment_amount,
            token_display_name,
            token_ticker,
            num_decimals,
            None,
        );
    }

    #[only_owner]
    #[payable("EGLD")]
    #[endpoint(registerUnstakeToken)]
    fn register_unstake_token(
        &self,
        token_display_name: ManagedBuffer,
        token_ticker: ManagedBuffer,
        num_decimals: usize,
    ) {
        let payment_amount = self.call_value().egld_value().clone_value();
        self.unstake_token().issue_and_set_all_roles(
            EsdtTokenType::NonFungible,
            payment_amount,
            token_display_name,
            token_ticker,
            num_decimals,
            None,
        );
    }

    #[only_owner]
    #[endpoint(setStateActive)]
    fn set_state_active(&self) {
        self.state().set(State::Active);
    }

    #[only_owner]
    #[endpoint(setStateInactive)]
    fn set_state_inactive(&self) {
        self.state().set(State::Inactive);
    }

    #[inline]
    fn is_state_active(&self, state: State) -> bool {
        state == State::Active
    }

    #[view(getState)]
    #[storage_mapper("state")]
    fn state(&self) -> SingleValueMapper<State>;

    #[view(getLsTokenId)]
    #[storage_mapper("lsTokenId")]
    fn ls_token(&self) -> FungibleTokenMapper<Self::Api>;

    #[view(getMainToken)]
    #[storage_mapper("mainToken")]
    fn main_token(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(getLsSupply)]
    #[storage_mapper("lsTokenSupply")]
    fn ls_token_supply(&self) -> SingleValueMapper<BigUint>;

    #[view(getVirtualXOXNOReserve)]
    #[storage_mapper("virtualXOXNOReserve")]
    fn virtual_xoxno_reserve(&self) -> SingleValueMapper<BigUint>;

    #[view(getTotalWithdrawnXOXNO)]
    #[storage_mapper("totalWithdrawnXOXNO")]
    fn total_withdrawn_xoxno(&self) -> SingleValueMapper<BigUint>;

    #[view(getUnstakeTokenId)]
    #[storage_mapper("unstakeTokenId")]
    fn unstake_token(&self) -> NonFungibleTokenMapper<Self::Api>;

    #[view(getUnstakeTokenSupply)]
    #[storage_mapper("unstakeTokenSupply")]
    fn unstake_token_supply(&self) -> SingleValueMapper<BigUint>;
}
