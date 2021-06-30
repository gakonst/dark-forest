use ethers::types::Address;
use crate::utils::addr;

/// The networks where the Dark Forest contracts are deployed at
pub enum Network {
    /// [XDai](https://www.xdaichain.com/) is a PoA Ethereum sidechain using DAI as its fee token
    /// RPC Endpoints can be found [here](https://www.xdaichain.com/for-developers/developer-resources#json-rpc-endpoints)
    Xdai,
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
    /// The Dark Forest [GPT3 contract](https://github.com/darkforest-eth/eth/blob/58a529bdbb8fd2645f00424f28f86bd481a36822/contracts/DarkForestGPTCredit.sol)
    pub gpt: Address,
    /// The Dark Forest [utils library](https://github.com/darkforest-eth/eth/blob/58a529bdbb8fd2645f00424f28f86bd481a36822/contracts/DarkForestUtils.sol)
    pub utils_lib: Address,
    pub planet_lib: Address,
    pub artifact_utils_lib: Address,
    pub verifier_lib: Address,
    pub initialize_lib: Address,
    pub lazy_update_lib: Address,
}

// https://cdn.skypack.dev/-/@darkforest_eth/contracts@v6.2.9-beYRQYHGj7ONrOAiAcwi/dist=es2020,mode=imports/optimized/@darkforest_eth/contracts.js
impl Config {
    /// Provides the smart contract addresses for Dark Forest 0.6r2 on xDAI
    pub fn xdai() -> Self {
        Self {
            start_block: 16750188,
            core: addr("0x0F8E1785f850d4bF1155614883c2cB778a69c463"),
            tokens: addr("0xafb1A0C81c848Ad530766aD4BE2fdddC833e1e96"),
            getters: addr("0xb85aAAB2272A95Fd3E929cF8ad16a5EC012c7bfA"),
            whitelist: addr("0x71CB56Dbb60ef54eA639BE325f5BD4295e09aAb6"),
            gpt: addr("0x399df8CD854F0C875A8ce770d29f39093bE4a18f"),
            utils_lib: addr("0x56929362F772146CfeB7b08f4eDA7a6F7cfa51B7"),
            planet_lib: addr("0x6Db5d1D44EF9ffD308238914C9C8fC949EEd4Bf7"),
            artifact_utils_lib: addr("0x590b47Ea6Cb629AD8a9228886e6C1DAA7a31630c"),
            verifier_lib: addr("0xF5c5c86B592a00d2d68958ABA4A316A4B0eBaDac"),
            initialize_lib: addr("0x26ddc1F8119B5A358Dd501eC167d6380e38c0e31"),
            lazy_update_lib: addr("0x56603474D7FfAFD6A03c7D5bB7E87B7b99e7f536"),
        }
    }
}
