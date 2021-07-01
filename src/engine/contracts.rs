//! Smart Contracts APIs
use std::{convert::TryFrom, sync::Arc};

use ethers::{
    contract::ContractError,
    providers::{Http, Middleware, Provider},
    types::{TxHash, U256},
};

use crate::{
    bindings::{DarkForestCore, DarkForestGetters},
    config::{Config, Network},
    types::planet::{PlanetId, PlanetInfo, PlanetLocation, UpgradeBranch},
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

    /// Given a planet's ID, return the planet w/ any non-initialized values set to its defaults
    pub async fn planet_initialized(&self, id: PlanetId) -> Result<PlanetInfo, ContractError<M>> {
        let planets = self
            .getters
            .bulk_get_planets_data_by_ids(vec![*id.as_ref()])
            .call()
            .await?;
        let planet = PlanetInfo::from(planets[0]);
        Ok(planet)
    }

    /// Given a planet's ID and upgrade branch, upgrade that planet
    pub async fn upgrade_planet<T: AsRef<U256>>(
        &self,
        id: T,
        branch: UpgradeBranch,
    ) -> Result<TxHash, ContractError<M>> {
        // TODO: Can we improve ethers-rs APIs to be able to return a Pending Transaction
        // instead of derefing down to just the tx hash?
        let call = self.core.upgrade_planet(*id.as_ref(), branch.into());
        let pending_tx = call.send().await?;
        Ok(*pending_tx)
    }

    /// Given a planet's ID, it prospects the planet (prepares it for finding an artifact)
    pub async fn prospect_planet<T: AsRef<U256>>(&self, id: T) -> Result<TxHash, ContractError<M>> {
        let call = self.core.prospect_planet(*id.as_ref());
        let pending_tx = call.send().await?;
        Ok(*pending_tx)
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
mod tests {
    use super::*;

    #[tokio::test]
    async fn get_planet() {
        let api = Contracts::new_xdai_readonly();
        let loc = "000094c6002fd43d80ce2853a8e77a3d00488b0694aae4e4fa0ddc534e5e7531"
            .parse::<U256>()
            .unwrap();
        let loc = PlanetId::from(loc);
        let planet = api.planet_initialized(loc).await.unwrap();
        dbg!(&planet);
    }
}
