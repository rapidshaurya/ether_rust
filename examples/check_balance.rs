use ethers::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    //here use your infura url
    let provider = Provider::<Http>::try_from(
        "https://kovan.infura.io/v3/a111fcada47746d990e0e2b7df50d00a"
    ).expect("could not instantiate HTTP Provider");
    

let adr1: Address = "0x436529019aE6B79e2389085cbB6ca1FD772f2fbD".parse()?;
// Not enough ether to pay for the transaction
let balance = provider.get_balance(adr1,None).await?;
println!("Balance of {} is : {}", adr1, balance);

Ok(())
}