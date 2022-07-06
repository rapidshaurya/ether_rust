# ether_rust
This repo will help you in using ether.rs crate with the help of some examples.
## Run example ##
Enter in terminal :
- cargo run --example contract_read (this example will help you how to read data using contract function)
- cargo run --example check_balance (this example will help you how to print or get balance of particular account)
- cargo run --example watch (this example will help to get latest block details from etherscan)
- cargo run --example uniswapv2 (this example will help you with uniswapV2)
- cargo run --example uniswapv2_tx (This example will show you how to swap the WETH token with the DAI token)
## Links
- [ethers-rs github link](https://github.com/gakonst/ethers-rs)
- [Ganache link](https://trufflesuite.com/ganache/)
- [Remix IDE link](https://remix.ethereum.org/)
- [Infura for creating your infura mainnet url](https://infura.io/)
- [Metamask for creating wallet](https://metamask.io/)
### Note
- For running my uniswapv2_tx example, you should use your wallet private key and infura url for making a successful transaction
- To make a successful transaction to testnetwork, add the dependencies "ethers=version="0.13.0", features = ["legacy"]" to cargo.toml.