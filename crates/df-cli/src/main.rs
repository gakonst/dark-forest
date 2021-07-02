use color_eyre::eyre::Result;
use ethers::providers::{Http, Provider};
use std::{collections::HashMap, convert::TryFrom, path::PathBuf, sync::Arc};
use structopt::StructOpt;

use dark_forest::{engine::Contracts, types::Map, contracts::Network};

#[derive(Debug, StructOpt)]
struct Opts {
    // https://www.xdaichain.com/for-developers/developer-resources#json-rpc-endpoints
    #[structopt(default_value = "https://dai.poa.network")]
    url: String,

    #[structopt(default_value = "./test-vectors/large-map.json")]
    map: PathBuf,
}

#[tokio::main]
async fn main() -> Result<()> {
    let opts = Opts::from_args();

    let provider = Provider::<Http>::try_from(opts.url.as_str())?;
    let provider = Arc::new(provider);

    let contracts = Contracts::new(provider, Network::Xdai);

    let map = std::fs::read_to_string(opts.map)?;
    let map: Map = serde_json::from_str(&map)?;

    let mut locs = map.planets();

    while !locs.is_empty() {
        let ids = locs.drain(..100).collect::<Vec<_>>();
        let planets = contracts.planets_with_defaults(ids.clone()).await?;

        let planets = planets
            .zip(ids)
            .map(|(planet, id)| (id, planet))
            .collect::<HashMap<_, _>>();

        dbg!(planets);
    }
    Ok(())
}
