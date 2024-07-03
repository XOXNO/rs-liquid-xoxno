ADDRESS=erd1qqqqqqqqqqqqqpgqs5w0wfmf5gw7qae82upgu26cpk2ug8l245qszu3dxf
PROXY=https://gateway.xoxno.com
PROJECT="./output-docker/rs-liquid-xoxno/rs-liquid-xoxno.wasm"

deploy() {
    mxpy contract deploy --bytecode=${PROJECT} --arguments str:XOXNO-c1293a --recall-nonce \
    --ledger --ledger-account-index=0 --ledger-address-index=7 \
    --gas-limit=150000000 --send --proxy=${PROXY} --chain=1 || return

    echo "New smart contract address: ${ADDRESS}"
}

upgrade() {
    echo "Upgrade smart contract address: ${ADDRESS}"
    mxpy  contract upgrade ${ADDRESS} --bytecode=${PROJECT} --recall-nonce \
    --ledger --ledger-account-index=0 --ledger-address-index=7 \
    --gas-limit=55000000 --send --proxy=${PROXY} --chain="1" || return
}

registerLsToken() {
    mxpy contract call ${ADDRESS} --recall-nonce --function="registerLsToken" \
    --arguments str:LXOXNO str:LXOXNO 0x12 --value 50000000000000000 \
    --ledger --ledger-account-index=0 --ledger-address-index=7 \
    --gas-limit=150000000 --send --proxy=${PROXY} --chain=1 || return
}

registerUnstakeToken() {
    mxpy contract call ${ADDRESS} --recall-nonce --function="registerUnstakeToken" \
    --arguments str:UXOXNO str:UXOXNO 0x00 --value 50000000000000000 \
    --ledger --ledger-account-index=0 --ledger-address-index=7 \
    --gas-limit=150000000 --send --proxy=${PROXY} --chain=1 || return
}

setStateActive() {
    mxpy contract call ${ADDRESS} --recall-nonce --function="setStateActive" \
    --ledger --ledger-account-index=0 --ledger-address-index=7 \
    --gas-limit=15000000 --send --proxy=${PROXY} --chain=1 || return
}

getExchangeRate() {
    mxpy --verbose contract query ${ADDRESS} \
        --proxy=${PROXY} \
        --function="getExchangeRate"
}

getLsTokenAmountForMainTokenAmount() {
    mxpy --verbose contract query ${ADDRESS} \
        --proxy=${PROXY} \
        --function="getLsTokenAmountForMainTokenAmount" --arguments 1000000000000000000
}

getMainTokenAmountForPosition() {
    mxpy --verbose contract query ${ADDRESS} \
        --proxy=${PROXY} \
        --function="getMainTokenAmountForPosition" --arguments 892262748273425358
}

verifyContract() {
    mxpy --verbose contract verify "${ADDRESS}"  \
    --packaged-src=./output-docker/rs-liquid-xoxno/rs-liquid-xoxno-0.0.0.source.json --verifier-url="https://play-api.multiversx.com" \
    --docker-image="multiversx/sdk-rust-contract-builder:v8.0.0" --ledger --ledger-account-index=0 --ledger-address-index=7  || return 
}