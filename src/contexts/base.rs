use crate::liquidity_pool::State;

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

pub struct StorageCache<'a, C>
where
    C: crate::config::ConfigModule,
{
    sc_ref: &'a C,
    pub contract_state: State,
    pub main_token_id: TokenIdentifier<C::Api>,
    pub ls_token_id: TokenIdentifier<C::Api>,
    pub ls_token_supply: BigUint<C::Api>,
    pub virtual_xoxno_reserve: BigUint<C::Api>,
    pub total_withdrawn_xoxno: BigUint<C::Api>,
}

impl<'a, C> StorageCache<'a, C>
where
    C: crate::config::ConfigModule,
{
    pub fn new(sc_ref: &'a C) -> Self {
        StorageCache {
            contract_state: sc_ref.state().get(),
            main_token_id: sc_ref.main_token().get(),
            ls_token_id: sc_ref.ls_token().get_token_id(),
            ls_token_supply: sc_ref.ls_token_supply().get(),
            virtual_xoxno_reserve: sc_ref.virtual_xoxno_reserve().get(),
            total_withdrawn_xoxno: sc_ref.total_withdrawn_xoxno().get(),
            sc_ref,
        }
    }
}

impl<'a, C> Drop for StorageCache<'a, C>
where
    C: crate::config::ConfigModule,
{
    fn drop(&mut self) {
        // commit changes to storage for the mutable fields
        self.sc_ref.ls_token_supply().set(&self.ls_token_supply);
        self.sc_ref
            .virtual_xoxno_reserve()
            .set(&self.virtual_xoxno_reserve);
        self.sc_ref
            .total_withdrawn_xoxno()
            .set(&self.total_withdrawn_xoxno);
    }
}
