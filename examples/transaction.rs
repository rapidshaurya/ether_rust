use ethers::{prelude::*};
use eyre::Result;
use std::convert::TryFrom;

#[tokio::main]
async fn main() -> Result<()> {
    let provider = Provider::<Http>::try_from(
        "HTTP://127.0.0.1:7545"     // Ganache local server url
    ).expect("could not instantiate HTTP Provider");

    // storing sender account address in from variable and receiver account address in to variable
    let from = "0x39A08652FFE1312fAAC563074bc5db745f71eF4e".parse::<Address>()?;
    let to = "0x3F50A27228457396ABF5b18D08C220b030214a67".parse::<Address>()?;

    let val: i128=1000000000000000000; // 1 ether = 1000000000000000000 wei
    // craft the tx
    // TranactionRequest function take value in wei
    let tx = TransactionRequest::new().to(to).value(val).from(from); // specify the `from` field so that the client knows which account to use

    // storing nonce and balance of sender
    let balance_before = provider.get_balance(from, None).await?;
    let nonce1 = provider.get_transaction_count(from, None).await?;

    // broadcast it via the eth_sendTransaction API
    let tx = provider.send_transaction(tx, None).await?.await?;

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