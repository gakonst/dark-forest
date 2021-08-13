use crate::addr;
use ethers::types::Address;

/// The networks where the Dark Forest contracts are deployed at
pub enum Network {
    /// [XDai](https://www.xdaichain.com/) is a PoA Ethereum sidechain using DAI as its fee token
    /// RPC Endpoints can be found [here](https://www.xdaichain.com/for-developers/developer-resources#json-rpc-endpoints)
    Xdai,
    Custom(Config),
}

/// Contains all the addresses of the Dark Forest contracts and libraries, along
/// with the block at which they were deployed
pub struct Config {
    /// The deployment block
    pub start_block: u64,
    /// The Dark Forest [core contract](https://github.com/darkforest-eth/eth/blob/58a529bdbb8fd2645f00424f28f86bd481a36822/contracts/DarkForestCore.sol)
    pub core: Address,
    /// The Dark Forest [tokens contract](https://github.com/darkforest-eth/eth/blob/58a529bdbb8fd2645f00424f28f86bd481a36822/contracts/DarkForestTokens.sol)
    pub tokens: Address,
    /// The Dark Forest [getters contract](https://github.com/darkforest-eth/eth/blob/58a529bdbb8fd2645f00424f28f86bd481a36822/contracts/DarkForestGetters.sol)
    pub getters: Address,
    /// The Dark Forest [whitelist contract](https://github.com/darkforest-eth/eth/blob/58a529bdbb8fd2645f00424f28f86bd481a36822/contracts/Whitelist.sol)
    pub whitelist: Address,
}

// https://cdn.skypack.dev/-/@darkforest_eth/contracts@v6.2.9-beYRQYHGj7ONrOAiAcwi/dist=es2020,mode=imports/optimized/@darkforest_eth/contracts.js
impl Config {
    /// Provides the smart contract addresses for Dark Forest 0.6r2 on xDAI
    pub fn xdai() -> Self {
        Self {
            start_block: 16750188,
            core: addr("0x71340606E119de5a4dc3f1CA6bc5ec102a7fA7c0"),
            tokens: addr("0x621ce133521c3B1cf11C0b9423406F01835af0ee"),
            getters: addr("0x6C5b4B9b835e4e085C5B8124E44045D5b3b57Cd9"),
            whitelist: addr("0xc278c106777E4a3a51FAdD5fF2489394Af0ACddA"),
        }
    }
}
