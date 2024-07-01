use multiversx_sc_scenario::imports::*;
use rs_liquid_xoxno::*;
use rs_xoxno_proxy::*;

pub const OWNER_ADDRESS: TestAddress = TestAddress::new("owner");
pub const DELEGATOR_ADDRESS: TestAddress = TestAddress::new("delegator");
pub const RS_LIQUIDXOXNO_ADDRESS: TestSCAddress = TestSCAddress::new("rs_liquid_xoxno");
pub const CODE_PATH: MxscPath = MxscPath::new("output/rs-liquid-xoxno.mxsc.json");
pub const MAIN_TOKEN_ID: TestTokenIdentifier = TestTokenIdentifier::new("XOXNO-123456");
pub const LS_TOKEN_ID: TestTokenIdentifier = TestTokenIdentifier::new("LXOXNO-123456");
pub const UNBOUND_TOKEN_ID: TestTokenIdentifier = TestTokenIdentifier::new("UXOXNO-123456");

pub fn init_world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.register_contract(CODE_PATH, rs_liquid_xoxno::ContractBuilder);
    blockchain
}

pub fn deploy_contract(world: &mut ScenarioWorld) -> TestSCAddress {
    let liquid_sc = world.code_expression(&CODE_PATH.eval_to_expr());
    let mut acc = Account::new().code(liquid_sc).owner(OWNER_ADDRESS);

    acc.storage.insert(
        b"lsTokenId".to_vec().into(),
        b"LXOXNO-123456".to_vec().into(),
    );

    acc.storage.insert(
        b"mainToken".to_vec().into(),
        b"XOXNO-123456".to_vec().into(),
    );

    acc.storage.insert(
        b"unstakeTokenId".to_vec().into(),
        b"UXOXNO-123456".to_vec().into(),
    );

    world.set_state_step(
        SetStateStep::new()
            .put_account(RS_LIQUIDXOXNO_ADDRESS, acc)
            .block_epoch(1),
    );

    world.set_esdt_local_roles(
        RS_LIQUIDXOXNO_ADDRESS,
        b"LXOXNO-123456",
        &[EsdtLocalRole::Mint, EsdtLocalRole::Burn],
    );

    world.set_esdt_local_roles(
        RS_LIQUIDXOXNO_ADDRESS,
        b"UXOXNO-123456",
        &[
            EsdtLocalRole::Mint,
            EsdtLocalRole::Burn,
            EsdtLocalRole::NftBurn,
            EsdtLocalRole::NftCreate,
            EsdtLocalRole::NftAddUri,
        ],
    );
    RS_LIQUIDXOXNO_ADDRESS
}

pub fn set_contract_state(world: &mut ScenarioWorld, state: State) {
    match state {
        State::Active => {
            world
                .tx()
                .from(OWNER_ADDRESS)
                .to(RS_LIQUIDXOXNO_ADDRESS)
                .typed(rs_xoxno_proxy::RsLiquidXoxnoProxy)
                .set_state_active()
                .run();
        }
        State::Inactive => {
            world
                .tx()
                .from(OWNER_ADDRESS)
                .to(RS_LIQUIDXOXNO_ADDRESS)
                .typed(rs_xoxno_proxy::RsLiquidXoxnoProxy)
                .set_state_inactive()
                .run();
        }
    }
}

pub fn add_liquidity(world: &mut ScenarioWorld, token_amount: u64) {
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
        .run();
}

pub fn remove_liquidity(world: &mut ScenarioWorld, token_amount: u64) {
    world
        .tx()
        .from(DELEGATOR_ADDRESS)
        .to(RS_LIQUIDXOXNO_ADDRESS)
        .typed(rs_xoxno_proxy::RsLiquidXoxnoProxy)
        .un_delegate()
        .egld_or_single_esdt(
            &EgldOrEsdtTokenIdentifier::esdt(LS_TOKEN_ID),
            0u64,
            &multiversx_sc::proxy_imports::BigUint::from(token_amount),
        )
        .run();
}

pub fn set_users(world: &mut ScenarioWorld) {
    world
        .account(DELEGATOR_ADDRESS)
        .nonce(1)
        // .esdt_balance(LS_TOKEN_ID, 1000u64)
        .esdt_balance(MAIN_TOKEN_ID, 1000u64);
    world
        .account(OWNER_ADDRESS)
        .nonce(1)
        .esdt_balance(MAIN_TOKEN_ID, 1000u64);
}

pub fn add_rewards(world: &mut ScenarioWorld, token_amount: u64) {
    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(RS_LIQUIDXOXNO_ADDRESS)
        .typed(rs_xoxno_proxy::RsLiquidXoxnoProxy)
        .add_rewards()
        .egld_or_single_esdt(
            &EgldOrEsdtTokenIdentifier::esdt(MAIN_TOKEN_ID),
            0u64,
            &multiversx_sc::proxy_imports::BigUint::from(token_amount),
        )
        .run();
}

pub fn withdraw_nft(world: &mut ScenarioWorld, nonce: u64) {
    world
        .tx()
        .from(DELEGATOR_ADDRESS)
        .to(RS_LIQUIDXOXNO_ADDRESS)
        .typed(rs_xoxno_proxy::RsLiquidXoxnoProxy)
        .withdraw()
        .egld_or_single_esdt(
            &EgldOrEsdtTokenIdentifier::esdt(UNBOUND_TOKEN_ID),
            nonce,
            &multiversx_sc::proxy_imports::BigUint::from(1u64),
        )
        .run();
}

pub fn check_ls_token_supply(world: &mut ScenarioWorld, expected_amount: u64) {
    world
        .query()
        .to(RS_LIQUIDXOXNO_ADDRESS)
        .typed(rs_xoxno_proxy::RsLiquidXoxnoProxy)
        .ls_token_supply()
        .returns(ExpectValue(expected_amount))
        .run();
}

pub fn check_virtual_xoxno_reserve(world: &mut ScenarioWorld, expected_amount: u64) {
    world
        .query()
        .to(RS_LIQUIDXOXNO_ADDRESS)
        .typed(rs_xoxno_proxy::RsLiquidXoxnoProxy)
        .virtual_xoxno_reserve()
        .returns(ExpectValue(expected_amount))
        .run();
}

pub fn check_unstake_token_supply(world: &mut ScenarioWorld, expected_amount: u64) {
    world
        .query()
        .to(RS_LIQUIDXOXNO_ADDRESS)
        .typed(rs_xoxno_proxy::RsLiquidXoxnoProxy)
        .unstake_token_supply()
        .returns(ExpectValue(expected_amount))
        .run();
}

pub fn check_balance(
    world: &mut ScenarioWorld,
    address: TestAddress,
    token_id: TestTokenIdentifier,
    expected_balance: u64,
) {
    world
        .check_account(address)
        .esdt_balance(token_id, expected_balance);
}

pub fn check_contract_state(world: &mut ScenarioWorld, expected_state: State) {
    world
        .query()
        .to(RS_LIQUIDXOXNO_ADDRESS)
        .typed(rs_xoxno_proxy::RsLiquidXoxnoProxy)
        .state()
        .returns(ExpectValue(expected_state))
        .run();
}

pub fn check_main_token(world: &mut ScenarioWorld, expected_token_id: TestTokenIdentifier) {
    world
        .query()
        .to(RS_LIQUIDXOXNO_ADDRESS)
        .typed(rs_xoxno_proxy::RsLiquidXoxnoProxy)
        .main_token()
        .returns(ExpectValue(expected_token_id))
        .run();
}
