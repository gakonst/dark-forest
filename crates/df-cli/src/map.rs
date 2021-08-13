use dark_forest::{engine::Contracts, types::Map};
use ethers::providers::Middleware;
use std::collections::HashMap;
use std::path::PathBuf;

use color_eyre::Result;

pub async fn print_map<M: Middleware + 'static>(
    map: PathBuf,
    contracts: Contracts<M>,
) -> Result<()> {
    let map = std::fs::read_to_string(&map)?;
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
