mod test_helpers;
use multiversx_sc::types::{EgldOrEsdtTokenIdentifier, TestAddress};
use multiversx_sc_scenario::{imports::SetStateStep, ExpectError, ScenarioTxRun};
use rs_liquid_xoxno::rs_xoxno_proxy::{self, State};
use test_helpers::*;

#[test]
fn rs_liquid_xoxno_initialization() {
    let mut world = init_world();

    world.start_trace();
    set_users(&mut world);

    // Deploy the contract
    let _new_address = deploy_contract(&mut world);

    // Query the main token to check initialization
    check_main_token(&mut world, MAIN_TOKEN_ID);
}

#[test]
fn test_set_contract_state() {
    let mut world = init_world();

    world.start_trace();
    world.account(OWNER_ADDRESS).nonce(1);

    // Deploy the contract
    let _new_address = deploy_contract(&mut world);

    // Check initial state (inactive)
    check_contract_state(&mut world, State::Inactive);

    // Set the contract state to active
    set_contract_state(&mut world, State::Active);

    // Check if the state is active
    check_contract_state(&mut world, State::Active);

    // Set the contract state to inactive
    set_contract_state(&mut world, State::Inactive);

    // Check if the state is inactive
    check_contract_state(&mut world, State::Inactive);
}

#[test]
fn test_add_initial_liquidity() {
    let mut world = init_world();

    world.start_trace();
    set_users(&mut world);

    // Deploy the contract
    deploy_contract(&mut world);

    // Set the contract state to active
    set_contract_state(&mut world, State::Active);

    // Define the token amount for adding liquidity
    let token_amount = 1000u64;
    add_liquidity(&mut world, token_amount);

    // Query to check the LS token supply
    check_ls_token_supply(&mut world, token_amount);

    // Query to check the original token staked amount
    check_virtual_xoxno_reserve(&mut world, token_amount);
}

#[test]
fn test_remove_liquidity() {
    let mut world = init_world();

    world.start_trace();
    set_users(&mut world);

    // Deploy the contract
    let _new_address = deploy_contract(&mut world);

    // Set the contract state to active
    set_contract_state(&mut world, State::Active);

    // Define the token amount for adding liquidity
    let token_amount = 1000u64;
    add_liquidity(&mut world, token_amount);

    // Query to check the LS token supply
    check_ls_token_supply(&mut world, token_amount);

    // Define the token amount for removing liquidity
    let amount_to_remove = 500u64;
    // Remove liquidity
    remove_liquidity(&mut world, amount_to_remove);

    // Query to check the original token staked amount after withdraw
    check_virtual_xoxno_reserve(&mut world, token_amount - amount_to_remove);
}

#[test]
fn test_add_rewards() {
    let mut world = init_world();

    world.start_trace();
    set_users(&mut world);

    // Deploy the contract
    let _new_address = deploy_contract(&mut world);

    // Set the contract state to active
    set_contract_state(&mut world, State::Active);

    // Define the token amount for adding liquidity
    let token_amount = 500u64;
    add_liquidity(&mut world, token_amount);

    // Query to check the LS token supply
    check_ls_token_supply(&mut world, token_amount);

    check_balance(&mut world, DELEGATOR_ADDRESS, LS_TOKEN_ID, 500u64);

    // Define the rewards amount
    let rewards = 500u64;
    add_rewards(&mut world, rewards);

    // Add more liquidity
    add_liquidity(&mut world, token_amount);

    check_balance(&mut world, DELEGATOR_ADDRESS, LS_TOKEN_ID, 750u64);

    // Query to check the LS token supply including rewards
    check_ls_token_supply(&mut world, 750u64);

    // Define the amount to remove
    let amount_to_remove = 50u64;
    // Remove liquidity
    remove_liquidity(&mut world, amount_to_remove);

    check_balance(&mut world, DELEGATOR_ADDRESS, LS_TOKEN_ID, 700u64);

    // Query to check the virtual XOXNO reserve
    check_virtual_xoxno_reserve(&mut world, 1400u64);

    // Query to check the unstake token supply
    check_unstake_token_supply(&mut world, 100u64);

    // Query to check the LS token supply after removing liquidity
    check_ls_token_supply(&mut world, 700u64);
}

#[test]
fn test_withdraw() {
    let mut world = init_world();

    world.start_trace();
    set_users(&mut world);

    // Deploy the contract
    let _new_address = deploy_contract(&mut world);

    // Set the contract state to active
    set_contract_state(&mut world, State::Active);

    // Define the token amount for adding liquidity
    let token_amount = 1000u64;
    add_liquidity(&mut world, token_amount);

    // Define the amount to remove
    let amount_to_remove = 550u64;
    // Remove liquidity
    remove_liquidity(&mut world, amount_to_remove);

    // Attempt to withdraw before the unstake period has passed
    world
        .tx()
        .from(DELEGATOR_ADDRESS)
        .to(RS_LIQUIDXOXNO_ADDRESS)
        .typed(rs_xoxno_proxy::RsLiquidXoxnoProxy)
        .withdraw()
        .egld_or_single_esdt(
            &EgldOrEsdtTokenIdentifier::esdt(UNBOUND_TOKEN_ID),
            1u64,
            &multiversx_sc::proxy_imports::BigUint::from(1u64),
        )
        .returns(ExpectError(4, "The unstake period has not passed"))
        .run();

    // Advance the block epoch to simulate the passage of time
    world.set_state_step(SetStateStep::new().block_epoch(11));

    // Withdraw the tokens after the unstake period has passed
    withdraw_nft(&mut world, 1);

    // Check the balance of the main token after withdrawal
    check_balance(&mut world, DELEGATOR_ADDRESS, MAIN_TOKEN_ID, 550u64);
}

#[test]
fn test_state_transitions() {
    let mut world = init_world();

    world.start_trace();
    set_users(&mut world);

    // Deploy the contract
    let _new_address = deploy_contract(&mut world);

    // Try adding liquidity in inactive state (should fail)
    let token_amount = 1000u64;
    world
        .tx()
        .from(DELEGATOR_ADDRESS)
        .to(RS_LIQUIDXOXNO_ADDRESS)
        .typed(rs_xoxno_proxy::RsLiquidXoxnoProxy)
        .delegate(multiversx_sc::proxy_imports::OptionalValue::<TestAddress>::None)
        .egld_or_single_esdt(
            &EgldOrEsdtTokenIdentifier::esdt(MAIN_TOKEN_ID),
            0u64,
            &multiversx_sc::proxy_imports::BigUint::from(token_amount),
        )
        .returns(ExpectError(4, "Not active"))
        .run();
}

#[test]
fn test_remove_exact_liquidity() {
    let mut world = init_world();

    world.start_trace();
    set_users(&mut world);

    // Deploy the contract
    deploy_contract(&mut world);

    // Set the contract state to active
    set_contract_state(&mut world, State::Active);

    // Define the token amount for adding liquidity
    let token_amount = 1000u64;
    add_liquidity(&mut world, token_amount);

    // Check the LS token supply
    check_ls_token_supply(&mut world, token_amount);

    // Remove exactly the same amount of liquidity
    remove_liquidity(&mut world, token_amount);

    // Check the LS token supply after removal
    check_ls_token_supply(&mut world, 0);

    // Check the original token staked amount after withdraw
    check_virtual_xoxno_reserve(&mut world, 0);
}

#[test]
fn test_add_minimum_liquidity() {
    let mut world = init_world();

    world.start_trace();
    set_users(&mut world);

    // Deploy the contract
    deploy_contract(&mut world);

    // Set the contract state to active
    set_contract_state(&mut world, State::Active);

    // Define the minimum token amount for adding liquidity
    let token_amount = 1u64;
    add_liquidity(&mut world, token_amount);

    // Check the LS token supply
    check_ls_token_supply(&mut world, token_amount);

    // Check the original token staked amount
    check_virtual_xoxno_reserve(&mut world, token_amount);
}

#[test]
fn test_remove_excess_liquidity() {
    let mut world = init_world();

    world.start_trace();
    world
        .account(DELEGATOR_ADDRESS)
        .nonce(1)
        .esdt_balance(LS_TOKEN_ID, 1500u64)
        .esdt_balance(MAIN_TOKEN_ID, 1000u64);
    world
        .account(OWNER_ADDRESS)
        .nonce(1)
        .esdt_balance(MAIN_TOKEN_ID, 1000u64);

    // Deploy the contract
    deploy_contract(&mut world);

    // Set the contract state to active
    set_contract_state(&mut world, State::Active);

    // Define the token amount for adding liquidity
    let token_amount = 1000u64;
    add_liquidity(&mut world, token_amount);

    // Check the LS token supply
    check_ls_token_supply(&mut world, token_amount);

    // Attempt to remove more liquidity than available
    let excess_amount = 1500u64;
    world
        .tx()
        .from(DELEGATOR_ADDRESS)
        .to(RS_LIQUIDXOXNO_ADDRESS)
        .typed(rs_xoxno_proxy::RsLiquidXoxnoProxy)
        .un_delegate()
        .egld_or_single_esdt(
            &EgldOrEsdtTokenIdentifier::esdt(LS_TOKEN_ID),
            0u64,
            &multiversx_sc::proxy_imports::BigUint::from(excess_amount),
        )
        .returns(ExpectError(4, "Not enough LP token supply"))
        .run();
}

#[test]
fn test_add_liquidity_various_amounts() {
    let mut world = init_world();

    world.start_trace();
    world
        .account(DELEGATOR_ADDRESS)
        .nonce(1)
        .esdt_balance(MAIN_TOKEN_ID, 1_500_000_000u64);
    world
        .account(OWNER_ADDRESS)
        .nonce(1)
        .esdt_balance(MAIN_TOKEN_ID, 1000u64);

    // Deploy the contract
    deploy_contract(&mut world);

    // Set the contract state to active
    set_contract_state(&mut world, State::Active);

    // Test adding a very small amount of tokens
    let small_amount = 1u64;
    add_liquidity(&mut world, small_amount);
    check_ls_token_supply(&mut world, small_amount);
    check_virtual_xoxno_reserve(&mut world, small_amount);

    // Test adding a very large amount of tokens
    let large_amount = 1_000_000_000u64;
    add_liquidity(&mut world, large_amount);
    check_ls_token_supply(&mut world, small_amount + large_amount);
    check_virtual_xoxno_reserve(&mut world, small_amount + large_amount);

    // Add a medium amount of tokens
    let medium_amount = 500_000u64;
    add_liquidity(&mut world, medium_amount);
    check_ls_token_supply(&mut world, small_amount + large_amount + medium_amount);
    check_virtual_xoxno_reserve(&mut world, small_amount + large_amount + medium_amount);
}

#[test]
fn test_remove_liquidity_various_amounts() {
    let mut world = init_world();

    world.start_trace();
    world
        .account(DELEGATOR_ADDRESS)
        .nonce(1)
        .esdt_balance(MAIN_TOKEN_ID, 1_000_000u64);
    world
        .account(OWNER_ADDRESS)
        .nonce(1)
        .esdt_balance(MAIN_TOKEN_ID, 1000u64);

    // Deploy the contract
    deploy_contract(&mut world);

    // Set the contract state to active
    set_contract_state(&mut world, State::Active);

    // Add an initial amount of liquidity
    let initial_amount = 1_000_000u64;
    add_liquidity(&mut world, initial_amount);
    check_ls_token_supply(&mut world, initial_amount);
    check_virtual_xoxno_reserve(&mut world, initial_amount);

    // Test removing a very small amount of LS tokens
    let small_remove_amount = 1u64;
    remove_liquidity(&mut world, small_remove_amount);
    check_ls_token_supply(&mut world, initial_amount - small_remove_amount);
    check_virtual_xoxno_reserve(&mut world, initial_amount - small_remove_amount);

    // Test removing a very large amount of LS tokens
    let large_remove_amount = 999_999u64;
    remove_liquidity(&mut world, large_remove_amount);
    check_ls_token_supply(
        &mut world,
        initial_amount - small_remove_amount - large_remove_amount,
    );
    check_virtual_xoxno_reserve(
        &mut world,
        initial_amount - small_remove_amount - large_remove_amount,
    );
}

#[test]
fn test_add_rewards_various_amounts() {
    let mut world = init_world();

    world.start_trace();
    world
        .account(DELEGATOR_ADDRESS)
        .nonce(1)
        .esdt_balance(MAIN_TOKEN_ID, 1_000_000u64);
    world
        .account(OWNER_ADDRESS)
        .nonce(1)
        .esdt_balance(MAIN_TOKEN_ID, 1_500_001u64);

    // Deploy the contract
    deploy_contract(&mut world);

    // Set the contract state to active
    set_contract_state(&mut world, State::Active);

    // Add an initial amount of liquidity
    let initial_amount = 1_000_000u64;
    add_liquidity(&mut world, initial_amount);
    check_ls_token_supply(&mut world, initial_amount);
    check_virtual_xoxno_reserve(&mut world, initial_amount);

    // Add a small reward
    let small_reward = 1u64;
    add_rewards(&mut world, small_reward);
    check_virtual_xoxno_reserve(&mut world, initial_amount + small_reward);

    // Add a large reward
    let large_reward = 1_000_000u64;
    add_rewards(&mut world, large_reward);
    check_virtual_xoxno_reserve(&mut world, initial_amount + small_reward + large_reward);

    // Add a medium reward
    let medium_reward = 500_000u64;
    add_rewards(&mut world, medium_reward);
    check_virtual_xoxno_reserve(
        &mut world,
        initial_amount + small_reward + large_reward + medium_reward,
    );
}

#[test]
fn test_add_rewards_and_observe_changes() {
    let mut world = init_world();

    world.start_trace();
    world
        .account(DELEGATOR_ADDRESS)
        .nonce(1)
        .esdt_balance(MAIN_TOKEN_ID, 2_100_000u64);
    world
        .account(OWNER_ADDRESS)
        .nonce(1)
        .esdt_balance(MAIN_TOKEN_ID, 600_001u64);

    // Deploy the contract
    deploy_contract(&mut world);

    // Set the contract state to active
    set_contract_state(&mut world, State::Active);

    // Add an initial amount of liquidity
    let initial_amount = 1_000_000u64;
    add_liquidity(&mut world, initial_amount);
    check_ls_token_supply(&mut world, initial_amount);
    check_virtual_xoxno_reserve(&mut world, initial_amount);

    // Add rewards and observe the changes
    let rewards = 500_000u64;
    add_rewards(&mut world, rewards);
    check_virtual_xoxno_reserve(&mut world, initial_amount + rewards);

    // Add more liquidity after rewards have been added
    let additional_amount = 1_000_000u64;
    add_liquidity(&mut world, additional_amount);

    // Calculate expected LP amount
    let total_xoxno = initial_amount + rewards;
    let expected_lp_amount = additional_amount * initial_amount / total_xoxno;

    // Check the LS token supply after adding more liquidity
    check_ls_token_supply(&mut world, initial_amount + expected_lp_amount);
    check_virtual_xoxno_reserve(&mut world, initial_amount + rewards + additional_amount);

    // Add another small reward
    let small_reward = 100_000u64;
    add_rewards(&mut world, small_reward);
    check_virtual_xoxno_reserve(
        &mut world,
        initial_amount + rewards + additional_amount + small_reward,
    );

    // Add more liquidity after the small reward
    let small_liquidity = 100_000u64;
    add_liquidity(&mut world, small_liquidity);

    // Calculate expected LP amount for the small liquidity addition
    let total_xoxno_after_rewards = initial_amount + rewards + additional_amount + small_reward;
    let expected_lp_amount_small =
        small_liquidity * (initial_amount + expected_lp_amount) / total_xoxno_after_rewards;

    // Check the LS token supply after adding small liquidity
    check_ls_token_supply(
        &mut world,
        initial_amount + expected_lp_amount + expected_lp_amount_small,
    );
    check_virtual_xoxno_reserve(
        &mut world,
        initial_amount + rewards + additional_amount + small_reward + small_liquidity,
    );
}
