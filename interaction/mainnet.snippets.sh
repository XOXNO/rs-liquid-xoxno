ADDRESS=
PROXY=https://gateway.xoxno.com
PROJECT="/Users/mihaieremia/GitHub/rs-liquid-xoxno/output/rs-liquid-xoxno.wasm"

deploy() {
    mxpy contract deploy --bytecode=${PROJECT} --recall-nonce \
    --ledger --ledger-account-index=0 --ledger-address-index=0 \
    --guardian erd1789rujqce0ya72k03h2jp3pgqf3vdtt0e8740tndfj0jstx3w78qxcewr8 \
    --guardian-service-url https://tools.multiversx.com/guardian \
    --guardian-2fa-code 730372 --version 2 --options 2 \
    --gas-limit=150000000 --send --proxy=${PROXY} --chain=1 || return

    echo "New smart contract address: ${ADDRESS}"
}

upgrade() {
    echo "Upgrade smart contract address: ${ADDRESS}"
    mxpy contract upgrade --metadata-payable ${ADDRESS} --bytecode=${PROJECT} --recall-nonce \
    --ledger --ledger-account-index=0 --ledger-address-index=0 \
    --gas-limit=150000000 --send --proxy=${PROXY} --chain=1 || return
}
