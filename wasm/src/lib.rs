// Code generated by the multiversx-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Upgrade:                              1
// Endpoints:                           18
// Async Callback:                       1
// Total number of exported functions:  21

#![no_std]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    rs_liquid_xoxno
    (
        init => init
        upgrade => upgrade
        delegate => add_liquidity
        unDelegate => remove_liquidity
        withdraw => withdraw
        addRewards => add_rewards
        getMainTokenAmountForPosition => get_ls_value_for_position
        getLsTokenAmountForMainTokenAmount => get_ls_amount_for_position
        registerLsToken => register_ls_token
        registerUnstakeToken => register_unstake_token
        setStateActive => set_state_active
        setStateInactive => set_state_inactive
        getState => state
        getLsTokenId => ls_token
        getMainToken => main_token
        getLsSupply => ls_token_supply
        getVirtualXOXNOReserve => virtual_xoxno_reserve
        getTotalWithdrawnXOXNO => total_withdrawn_xoxno
        getUnstakeTokenId => unstake_token
        getUnstakeTokenSupply => unstake_token_supply
    )
}

multiversx_sc_wasm_adapter::async_callback! { rs_liquid_xoxno }
