ADDRESS=erd1qqqqqqqqqqqqqpgq04vxf48vdlr97p3jz73qtxlf4l9p8rezah0s37nzrm
PROXY=https://devnet-gateway.xoxno.com
PROJECT="/Users/mihaieremia/GitHub/rs-liquid-xoxno/output/rs-liquid-xoxno.wasm"

deploy() {
    mxpy contract deploy --bytecode=${PROJECT} --arguments str:XOXNO-589e09 --recall-nonce \
    --ledger --ledger-account-index=0 --ledger-address-index=0 \
    --gas-limit=150000000 --send --proxy=${PROXY} --chain=D || return

    echo "New smart contract address: ${ADDRESS}"
}

upgrade() {
    echo "Upgrade smart contract address: ${ADDRESS}"
    mxpy  contract upgrade ${ADDRESS} --bytecode=${PROJECT} --recall-nonce \
    --ledger --ledger-account-index=0 --ledger-address-index=0 \
    --gas-limit=150000000 --send --proxy=${PROXY} --chain="D" || return
}

registerLsToken() {
    mxpy contract call ${ADDRESS} --recall-nonce --function="registerLsToken" \
    --arguments str:LXOXNO str:LXOXNO 0x12 --value 50000000000000000 \
    --ledger --ledger-account-index=0 --ledger-address-index=0 \
    --gas-limit=150000000 --send --proxy=${PROXY} --chain=D || return
}

registerUnstakeToken() {
    mxpy contract call ${ADDRESS} --recall-nonce --function="registerUnstakeToken" \
    --arguments str:UXOXNO str:UXOXNO 0x00 --value 50000000000000000 \
    --ledger --ledger-account-index=0 --ledger-address-index=0 \
    --gas-limit=150000000 --send --proxy=${PROXY} --chain=D || return
}

setStateActive() {
    mxpy contract call ${ADDRESS} --recall-nonce --function="setStateActive" \
    --ledger --ledger-account-index=0 --ledger-address-index=0 \
    --gas-limit=15000000 --send --proxy=${PROXY} --chain=D || return
}

