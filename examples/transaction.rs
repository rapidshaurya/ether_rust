use ethers::{prelude::*};
use eyre::Result;
use std::convert::TryFrom;

#[tokio::main]
async fn main() -> Result<()> {
    let provider = Provider::<Http>::try_from(
        "HTTP://127.0.0.1:7545"     // Ganache local server url
    ).expect("could not instantiate HTTP Provider");

    // storing sender account address in from variable and receiver account address in to variable
    let from = "0xdD3954F7b5fd7ee79512486518d3A027bb6CC0d4".parse::<Address>()?;
    let to = "0x0ccBD1739c1CFF6CBE9a41342aCbCFD2afb2BB75".parse::<Address>()?;

    let val: i128=1000000000000000000; // 1 ether = 1000000000000000000 wei
    // craft the tx
    // TranactionRequest function take value in wei
    let tx = TransactionRequest::new().to(to).value(val).from(from); // specify the `from` field so that the client knows which account to use

    // storing nonce and balance of sender
    let balance_before = provider.get_balance(from, None).await?;
    let nonce1 = provider.get_transaction_count(from, None).await?;

    // broadcast it via the eth_sendTransaction API
    let tx = provider.send_transaction(tx, None).await?.await?;
    //if you are using testnet instead of ganache or anvil then you have to add private key to wallet
    // for wallet you can go to uniswapv2_tx example
    // one thing more, you have to add    ethers={version ="0.13.0", features = ["legacy"]} in ethers dependencies
    println!("{}", serde_json::to_string(&tx)?);

    let nonce2 = provider.get_transaction_count(from, None).await?;

    assert!(nonce1 < nonce2);
    
    // storing balance of sender after payment
    let balance_after = provider.get_balance(from, None).await?;

    // assert if balance before payment is more than after payment
    assert!(balance_after < balance_before);

    //printing balance
    println!("Balance before {}", balance_before);
    println!("Balance after {}", balance_after);

    Ok(())
}