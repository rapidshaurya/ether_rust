/*
This example will help you to get the latest blocks which are getting mined in the etherscan.io mainnet.
*/
use ethers::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    
let provider = Provider::<Http>::try_from(
    "https://mainnet.infura.io/v3/a111fcada47746d990e0e2b7df50d00a"
).expect("could not instantiate HTTP Provider");

//use watch_block to get latest blocks
// .take(5) is use to get only 5 latest block's. You can can the value depending on your need.
let mut stream = provider.watch_blocks().await?.take(5);
    while let Some(block) = stream.next().await {
        let block = provider.get_block(block).await?.unwrap();
        println!(
            "Ts: {:?}\nblock number: {} \nAuthor: {:?}\nNounce: {:?}\nHash-> {:?}\n",
            block.timestamp,
            block.number.unwrap(),
            block.author.unwrap(),
            block.nonce.unwrap(),
            block.hash.unwrap()
        );
        
    }

    Ok(())

}