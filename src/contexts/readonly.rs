use crate::liquidity_pool::State;

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

pub struct ReadOnlyStorageCache<'a, C>
where
    C: crate::config::ConfigModule,
{
    _sc_ref: &'a C,
    pub contract_state: State,
    pub main_token_id: TokenIdentifier<C::Api>,
    pub ls_token_id: TokenIdentifier<C::Api>,
    pub ls_token_supply: BigUint<C::Api>,
    pub virtual_xoxno_reserve: BigUint<C::Api>,
    pub total_unstaked_xoxno: BigUint<C::Api>,
}

impl<'a, C> ReadOnlyStorageCache<'a, C>
where
    C: crate::config::ConfigModule,
{
    pub fn new(sc_ref: &'a C) -> Self {
        ReadOnlyStorageCache {
            contract_state: sc_ref.state().get(),
            main_token_id: sc_ref.main_token().get(),
            ls_token_id: sc_ref.ls_token().get_token_id(),
            ls_token_supply: sc_ref.ls_token_supply().get(),
            virtual_xoxno_reserve: sc_ref.virtual_xoxno_reserve().get(),
            total_unstaked_xoxno: sc_ref.unstake_token_supply().get(),
            _sc_ref: sc_ref,
        }
    }
}
