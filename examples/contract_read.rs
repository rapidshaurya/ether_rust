use ethers::prelude::*;
use eyre::Result;
use std::sync::Arc;

// Generate the type-safe contract bindings by providing the ABI
// definition in human readable format
abigen!(
    tethertoken,
    r#"[
        function balances(address) external view returns (uint256)
        function name() external view returns (string)
    ]"#,
    
);

#[tokio::main]
async fn main() -> Result<()> {
    let client = Provider::<Http>::try_from(
        "https://mainnet.infura.io/v3/a111fcada47746d990e0e2b7df50d00a",
    )?;
    let client = Arc::new(client);

    
    let contract_address = "0xdAC17F958D2ee523a2206206994597C13D831ec7".parse::<Address>()?;

    let data =tethertoken::new(contract_address, Arc::clone(&client));

    let account_address = "0x2b456255cd9820b1c471b1c80cc46AcCe8EB5acA".parse::<Address>()?;
    // getReserves -> get_reserves
    let reserve0 = data.balances(account_address).call().await?;
    let ret = data.name().call().await?;
    println!("Name: {}\nAccount Address: {}\nbalance: {}", ret, account_address, reserve0);

    Ok(())
}