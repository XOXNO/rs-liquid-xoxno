use crate::contexts::base::StorageCache;

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[type_abi]
#[derive(TopEncode)]
pub struct AddLiquidityEvent<M: ManagedTypeApi> {
    caller: ManagedAddress<M>,
    ls_token_id: TokenIdentifier<M>, // LXOXNO token
    ls_token_amount: BigUint<M>, // LXOXNO received after staking XOXNO
    ls_token_supply: BigUint<M>, // Current LXOXNO total supply after the current staking
    original_amount: BigUint<M>, // How much XOXNO was staked to receive the above LXOXNO amount
    virtual_xoxno_reserve: BigUint<M>, // Current XOXNO reserves (staked + rewards), including the new staking amount
    block: u64,
    epoch: u64,
    timestamp: u64,
}

#[type_abi]
#[derive(TopEncode)]
pub struct RemoveLiquidityEvent<M: ManagedTypeApi> {
    caller: ManagedAddress<M>,
    ls_token_id: TokenIdentifier<M>, // LXOXNO token
    ls_token_amount: BigUint<M>, // LXOXNO unstaked
    ls_token_supply: BigUint<M>, // LXOXNO total supply after unstake
    original_amount: BigUint<M>, // How much XOXNO will receive for this unstaked LXOXNO
    virtual_xoxno_reserve: BigUint<M>, // Current XOXNO reserves (staked + rewards) after unstake
    unbound_nft: EsdtTokenPayment<M>, // Full NFT data for the unbound
    block: u64,
    epoch: u64,
    timestamp: u64,
}

#[type_abi]
#[derive(TopEncode)]
pub struct AddRewardsEvent<M: ManagedTypeApi> {
    caller: ManagedAddress<M>,
    ls_token_id: TokenIdentifier<M>, // LXOXNO token
    ls_token_supply: BigUint<M>, // LXOXNO supply at current rewards event
    virtual_xoxno_reserve: BigUint<M>, // New XOXNO total reserve including the added rewards
    rewards_amount: BigUint<M>, // The amount of new XOXNO added as rewards
    block: u64,
    epoch: u64,
    timestamp: u64,
}

#[multiversx_sc::module]
pub trait EventsModule:
    crate::config::ConfigModule
    + multiversx_sc_modules::default_issue_callbacks::DefaultIssueCallbacksModule
{
    fn emit_delegate_event(
        &self,
        storage_cache: &StorageCache<Self>,
        caller: &ManagedAddress,
        ls_token_amount: &BigUint,
        original_amount: &BigUint,
    ) {
        let epoch = self.blockchain().get_block_epoch();
        self.add_liquidity_event(
            &storage_cache.ls_token_id,
            caller,
            epoch,
            &AddLiquidityEvent {
                caller: caller.clone(),
                ls_token_id: storage_cache.ls_token_id.clone(),
                ls_token_amount: ls_token_amount.clone(),
                ls_token_supply: storage_cache.ls_token_supply.clone(),
                original_amount: original_amount.clone(),
                virtual_xoxno_reserve: storage_cache.virtual_xoxno_reserve.clone(),
                block: self.blockchain().get_block_nonce(),
                epoch,
                timestamp: self.blockchain().get_block_timestamp(),
            },
        )
    }

    fn emit_remove_liquidity_event(
        &self,
        storage_cache: &StorageCache<Self>,
        unbound_nft: EsdtTokenPayment,
        ls_token_amount: &BigUint,
        unstake_token_amount: BigUint,
    ) {
        let epoch = self.blockchain().get_block_epoch();
        let caller = self.blockchain().get_caller();
        self.remove_liquidity_event(
            &storage_cache.ls_token_id,
            &caller,
            epoch,
            &RemoveLiquidityEvent {
                caller: caller.clone(),
                ls_token_id: storage_cache.ls_token_id.clone(),
                ls_token_amount: ls_token_amount.clone(),
                unbound_nft,
                original_amount: unstake_token_amount,
                ls_token_supply: storage_cache.ls_token_supply.clone(),
                virtual_xoxno_reserve: storage_cache.virtual_xoxno_reserve.clone(),
                block: self.blockchain().get_block_nonce(),
                epoch,
                timestamp: self.blockchain().get_block_timestamp(),
            },
        )
    }

    fn emit_add_rewards_event(
        &self,
        storage_cache: &StorageCache<Self>,
        caller: &ManagedAddress,
        reward_amount: &BigUint,
    ) {
        let epoch = self.blockchain().get_block_epoch();
        self.add_rewards_event(
            &storage_cache.ls_token_id,
            caller,
            epoch,
            &AddRewardsEvent {
                caller: caller.clone(),
                ls_token_id: storage_cache.ls_token_id.clone(),
                ls_token_supply: storage_cache.ls_token_supply.clone(),
                virtual_xoxno_reserve: storage_cache.virtual_xoxno_reserve.clone(),
                rewards_amount: reward_amount.clone(),
                block: self.blockchain().get_block_nonce(),
                epoch,
                timestamp: self.blockchain().get_block_timestamp(),
            },
        )
    }

    #[event("add_liquidity")]
    fn add_liquidity_event(
        &self,
        #[indexed] ls_token: &TokenIdentifier,
        #[indexed] caller: &ManagedAddress,
        #[indexed] epoch: u64,
        #[indexed] add_liquidity_event: &AddLiquidityEvent<Self::Api>,
    );

    #[event("remove_liquidity")]
    fn remove_liquidity_event(
        &self,
        #[indexed] ls_token: &TokenIdentifier,
        #[indexed] caller: &ManagedAddress,
        #[indexed] epoch: u64,
        #[indexed] remove_liquidity_event: &RemoveLiquidityEvent<Self::Api>,
    );

    #[event("add_rewards")]
    fn add_rewards_event(
        &self,
        #[indexed] main_token: &TokenIdentifier,
        #[indexed] caller: &ManagedAddress,
        #[indexed] epoch: u64,
        #[indexed] add_rewards_event: &AddRewardsEvent<Self::Api>,
    );
}
