//! Smart Contracts APIs
use std::sync::Arc;

use ethers::{contract::ContractError, providers::Middleware, types::U256};

use crate::{
    bindings::{DarkForestCore, DarkForestGetters},
    config::{Config, Network},
    types::planet::{Planet, PlanetInfo},
};

#[derive(Clone, Debug)]
/// Object containing all the necessary smart contracts
pub struct Contracts<M: Middleware> {
    pub core: DarkForestCore<M>,
    pub getters: DarkForestGetters<M>,
}

impl<M: Middleware> Contracts<M> {
    /// Creates a new contract object given an Ethers Middleware and a Network
    pub fn new(provider: Arc<M>, network: Network) -> Self {
        let cfg = match network {
            Network::Xdai => Config::xdai(),
        };

        Self {
            core: DarkForestCore::new(cfg.core, provider.clone()),
            getters: DarkForestGetters::new(cfg.getters, provider),
        }
    }

    /// Given a planet's ID, return the planet
    pub async fn planet<T: AsRef<U256>>(&self, id: T) -> Result<Planet, ContractError<M>> {
        let planet = self.core.planets(*id.as_ref()).call().await?;
        Ok(planet.into())
    }

    /// Given an iterator of planet IDs, return the planet infos
    pub async fn planets<I, T>(&self, ids: I) -> Result<Vec<PlanetInfo>, ContractError<M>>
    where
        T: AsRef<U256>,
        I: IntoIterator<Item = T>,
    {
        let ids: Vec<U256> = ids.into_iter().map(|id| *id.as_ref()).collect();

        // get the planets from the contract in a bulk
        let planets = self
            .getters
            .bulk_get_planets_data_by_ids(ids)
            .call()
            .await?;

        // convert them to our data type
        let planets = planets
            .into_iter()
            .map(|planet| PlanetInfo {
                planet: planet.0.into(),
                info: planet.1.into(),
                coords: planet.2.into(),
            })
            .collect::<Vec<_>>();

        Ok(planets)
    }
}
