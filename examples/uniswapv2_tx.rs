/*
In this example, I am using the IUniswapV2Pair contract to get the current WETH/DAI price.
This example will help you learn how to use the swapExactETHForTokens function to swap DAI tokens with WETH.
 */
use ethers::prelude::{abi::AbiEncode,*};
use eyre::Result;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use std::io;
use std::process;
// Generate the type-safe contract bindings by providing the ABI
// definition in human readable format
abigen!(
    UniswapV2Pair,
    r#"[
        function getReserves() external view returns (uint112 reserve0, uint112 reserve1, uint32 blockTimestampLast)
    ]"#,
);

abigen!(
    UniswapV2Router02,
    r#"[
        function WETH() external pure returns (address)
        function swapExactETHForTokens(uint amountOutMin, address[] calldata path, address to, uint deadline) external payable returns (uint[] memory amounts)
    ]"#,
    
);

// this function will give you valid timestamp
fn get_valid_timestamp(future_millis: u128) -> u128 {
    let start = SystemTime::now();
    let since_epoch = start.duration_since(UNIX_EPOCH).unwrap();
    let time_millis = since_epoch.as_millis().checked_add(future_millis).unwrap();
    time_millis
}


#[tokio::main]
async fn main() -> Result<()> {
    let client = Provider::<Http>::try_from(
        "https://mainnet.infura.io/v3/a111fcada47746d990e0e2b7df50d00a",
    )?;
    let client = Arc::new(client);

    // DAI/ETH pair on Uniswap V2   contract address
    let address = "0xA478c2975Ab1Ea89e8196811F51A7B7Ade33eB11".parse::<Address>()?;
    let pair = UniswapV2Pair::new(address, Arc::clone(&client));

    // getReserves -> get_reserves
    let (reserve0, reserve1, _timestamp) = pair.get_reserves().call().await?;
    println!("Reserves (WETH, DAI): ({}, {})", reserve0, reserve1);

    let mid_price = reserve0 as f64 / reserve1 as f64;
    println!("1 WETH = {} DAI ", mid_price);
    println!("Do you want to swap token?\nEnter your choice (y/n): ");

    let mut guess = String::new();
 
    io::stdin().read_line(&mut guess).expect("failed to readline");
    if guess.trim() == "y" {

        let client = Arc::new({
            // connect to the network
            let provider = Provider::<Http>::try_from(
                "https://kovan.infura.io/v3/a111fcada47746d990e0e2b7df50d00a",
            )?;
            //chain id
            let chain_id = provider.get_chainid().await?;
         
            // Here you can enter your wallet private
            let wallet = "a25f0586cb5caf7971733e58783286a277f91557209b38aef7e9e74c2fdf56e2"
                .parse::<LocalWallet>()?
                .with_chain_id(chain_id.as_u64());
            // connecting wallet to provider
            SignerMiddleware::new(provider, wallet)
        });
        //storing wallet account address into  "to" variable
        let to = client.address();
        let balance_before = client.get_balance(to, None).await?;
        println!("balance --> {} in wei",balance_before);
        // 0x1b02dA8Cb0d097eB8D57A175b88c7D8b47997506 Contract ADDRESS
        let contract_address = "0x1b02dA8Cb0d097eB8D57A175b88c7D8b47997506".parse::<Address>()?;

        let data =UniswapV2Router02::new(contract_address, Arc::clone(&client));

        let mut val = String::new();
        println!("Enter the amount of WETH you want to swap: ");
        io::stdin().read_line(&mut val).expect("failed to readline");
        let val= f64::powi(10.0, 18) *((val.trim()).parse::<f64>().unwrap());
        //converting float to U256
        let amount_out_min=u256_from_f64_saturating(mid_price);

        let val=u256_from_f64_saturating(val); 
        if balance_before<val {                                           
                                                                          
            println!("Insufficient Balance For Swapping!!!!");
            process::exit(0x0100);
        }
        // storing weth address in weth variable
        // 0x4F96Fe3b7A6Cf9725f59d353F723c1bDb64CA6Aa DAI ADDRESS
        
        let weth =data.weth().call().await?;
        println!("amount_out_min: {}", amount_out_min);
        let path: [H160; 2]=[weth, "0x4F96Fe3b7A6Cf9725f59d353F723c1bDb64CA6Aa".parse::<Address>()?];
        let path=path.to_vec();
        

        let valid_timestamp = get_valid_timestamp(300000); // storing unix time and adding 300000 in unix time
        let deadline =U256::from_dec_str(&valid_timestamp.to_string()).unwrap(); 
    
        let a=SwapExactETHForTokensCall{
            amount_out_min,
             path,
              to,
              deadline};
              // encoding the SwapExactETHForTokensCall data so that i can use in TransactionRequest function
              let b = SwapExactETHForTokensCall::encode(a);
                     
        let tx = TransactionRequest::new()
                      .to(contract_address)
                      .value(val)
                      .data(b);
    
        let _tx = client.send_transaction(tx, None).await?.await?;
         let balance_after = client.get_balance(to, None).await?;

        if balance_before <= balance_after {
            println!("Transaction Failed!!!");
        }
        else {
            println!("Transaction Successfull!!!\n{} <-- Current Balance", balance_after);
        }
    }
    else {
        println!("n is selected");
    }

    Ok(())
}