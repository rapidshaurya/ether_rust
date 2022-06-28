use ethers::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    
let provider = Provider::<Http>::try_from(
    "https://mainnet.infura.io/v3/6b7326134b194ca18b106fdc8bc8fb1a"
).expect("could not instantiate HTTP Provider");

//use watch_block to get latest blocks
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