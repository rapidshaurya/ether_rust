use ethers::prelude::*;
use eyre::Result;
use std::sync::Arc;

// Generate the type-safe contract bindings by providing the ABI
// definition in human readable format
abigen!(
    account123,
    r#"[
        function balances(address) external view returns (uint256)
        function name() external view returns (string)
        function transfer(address to, uint amount) returns (bool)
    ]"#,
    
);

#[tokio::main]
async fn main() -> Result<()> {
    let client = Provider::<Http>::try_from(
        "https://mainnet.infura.io/v3/c60b0bb42f8a4c6481ecd229eddaca27",
    )?;
    let client = Arc::new(client);

    // ETH/USDT pair on Uniswap V2
    let address = "0xdAC17F958D2ee523a2206206994597C13D831ec7".parse::<Address>()?;
    let data =account123::new(address, Arc::clone(&client));
    let ad1 = "0x2b456255cd9820b1c471b1c80cc46AcCe8EB5acA".parse::<Address>()?;
    // getReserves -> get_reserves
    let reserve0 = data.balances(ad1).call().await?;
    let r2 = data.name().call().await?;
    println!("Name: {}\nAccount address: {}\nbalance: {}", r2, ad1, reserve0);

    Ok(())
}