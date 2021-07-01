//! Smart Contracts APIs
use std::{convert::TryFrom, sync::Arc};

use ethers::{
    contract::ContractError,
    providers::{Middleware, Provider, Http},
    types::{TxHash, U256},
};

use crate::{
    bindings::{DarkForestCore, DarkForestGetters},
    config::{Config, Network},
    types::planet::{PlanetInfo, PlanetLocation /*, UpgradeBranch */},
};

#[derive(Clone, Debug)]
/// Object containing all the necessary smart contracts
pub struct Contracts<M: Middleware> {
    pub core: DarkForestCore<M>,
    pub getters: DarkForestGetters<M>,
}

impl Contracts<Provider<Http>> {
    pub fn new_xdai_readonly() -> Self {
        let provider = Provider::try_from("https://dai.poa.network").unwrap();
        Self::new(Arc::new(provider), Network::Xdai)
    }
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

    /// Given a planet's ID, return the planet w/ any non-initialized values set to its defaults
    pub async fn planet(&self, loc: PlanetLocation) -> Result<PlanetInfo, ContractError<M>> {
        Ok(self
            .planets_with_defaults([loc])
            .await?
            .next()
            .expect("planet not found"))
    }

    /// Given an iterator of planet IDs, return the planet infos
    pub async fn planets<I, T>(
        &self,
        ids: I,
    ) -> Result<impl Iterator<Item = PlanetInfo>, ContractError<M>>
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
        let planets = planets.into_iter().map(|planet| PlanetInfo {
            planet: planet.0.into(),
            info: planet.1.into(),
            coords: planet.2.into(),
        });

        Ok(planets)
    }

    /// Given an iterator of planet locations, return the planet infos w/ any non-initialized
    /// planets set to their default values
    pub async fn planets_with_defaults<I>(
        &self,
        locs: I,
    ) -> Result<impl Iterator<Item = PlanetInfo>, ContractError<M>>
    where
        I: IntoIterator<Item = PlanetLocation> + Clone,
    {
        let planets_iter = self.planets(locs.clone()).await?;

        let planets = planets_iter.zip(locs).map(|(planet, loc)| {
            if !planet.info.initialized {
                PlanetInfo::from(&loc)
            } else {
                planet
            }
        });

        Ok(planets)
    }
}

#[cfg(test)]
// TODO: Implement tests using Hardhat's docker mode
mod tests {
}
