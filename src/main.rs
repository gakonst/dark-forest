use color_eyre::eyre::Result;
use ethers::providers::{Http, Provider};
use std::{convert::TryFrom, path::PathBuf, sync::Arc};
use structopt::StructOpt;

use dark_forest::{engine::Contracts, types::Map, Network};

#[derive(Debug, StructOpt)]
struct Opts {
    // https://www.xdaichain.com/for-developers/developer-resources#json-rpc-endpoints
    #[structopt(default_value = "https://dai.poa.network")]
    url: String,

    #[structopt(default_value = "./test-vectors/map.json")]
    map: PathBuf,
}

#[tokio::main]
async fn main() -> Result<()> {
    let opts = Opts::from_args();

    let provider = Provider::<Http>::try_from(opts.url.as_str())?;
    let provider = Arc::new(provider);

    let contracts = Contracts::new(provider, Network::Xdai);

    let map = std::fs::read_to_string("./test-vectors/large-map.json")?;
    let map: Map = serde_json::from_str(&map)?;

    let mut locs = map.planets();

    while !locs.is_empty() {
        let ids = locs.drain(..50);
        let planets = contracts.planets(ids).await?;
        dbg!(planets);
    }
    Ok(())
}
