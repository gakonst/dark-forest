//! Smart Contracts APIs
use ark_circom::ethereum::Proof;
use df_contracts::{
    bindings::{DarkForestCore, DarkForestGetters},
    config::{Config, Network},
};
use df_types::{
    constants::{
        PERLIN_LENGTH_SCALE, PERLIN_MIRROR_X, PERLIN_MIRROR_Y, PLANETHASH_KEY, SPACETYPE_KEY,
    },
    planet::{PlanetId, PlanetInfo, PlanetLocation, UpgradeBranch},
};
use ethers::{
    contract::ContractError,
    prelude::builders::ContractCall,
    providers::{Http, Middleware, Provider},
    types::{TxHash, U256},
};
use std::{convert::TryFrom, sync::Arc};

#[derive(Clone)]
pub struct MoveArgs {
    pub population: U256,
    pub silver: U256,
    pub moved_artifact: Option<U256>,
}

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
            Network::Custom(inner) => inner,
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

    pub fn move_<P: Into<Proof>>(
        &self,
        from: &PlanetLocation,
        to: &PlanetLocation,
        args: &MoveArgs,
        proof: P,
    ) -> ContractCall<M, U256> {
        let (a, b, c) = to_eth_type(proof);

        // format the public inputs
        let input = [
            // locations
            from.hash.0,
            to.hash.0,
            // the ending perlin
            to.perlin.into(),
            // consistency checks
            to.max_radius().into(),
            to.max_distance(from).into(),
            // 5-9 are the game specific params
            // they did this so they dont need a new ceremony per round?
            PLANETHASH_KEY.into(),
            SPACETYPE_KEY.into(),
            PERLIN_LENGTH_SCALE.into(),
            (PERLIN_MIRROR_X as u8).into(),
            (PERLIN_MIRROR_Y as u8).into(),
            // Non-SNARK-related game args
            args.population,
            args.silver,
            args.moved_artifact.unwrap_or_else(U256::zero),
        ];

        self.core.move_(a, b, c, input)
    }

    pub fn initialize<P: Into<Proof>>(
        &self,
        location: &PlanetLocation,
        proof: P,
    ) -> ContractCall<M, U256> {
        let (a, b, c) = to_eth_type(proof);

        // format the public inputs
        let input = [
            location.hash.0,
            location.perlin.into(),
            location.max_radius().into(),
            PLANETHASH_KEY.into(),
            SPACETYPE_KEY.into(),
            PERLIN_LENGTH_SCALE.into(),
            (PERLIN_MIRROR_X as u8).into(),
            (PERLIN_MIRROR_Y as u8).into(),
        ];

        self.core.initialize_player(a, b, c, input)
    }
}

type Abc = ([U256; 2], [[U256; 2]; 2], [U256; 2]);
fn to_eth_type<P: Into<Proof>>(proof: P) -> Abc {
    // lay the proof in the correct order
    let proof = proof.into();
    let proof = proof.as_tuple();
    let a = [proof.0 .0, proof.0 .1];
    // b.as_tuple() already handles the reverse ordering in G2 elements
    let b = [proof.1 .0, proof.1 .1];
    let c = [proof.2 .0, proof.2 .1];
    (a, b, c)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        prover::{init::InitProver, mover::MoveProver, tests::circuit_prover},
        tests::{from_planet, root_path, to_planet},
        HOME,
    };
    use ark_std::rand::thread_rng;
    use df_contracts::addr;
    use ethers::prelude::{LocalWallet, SignerMiddleware};

    #[tokio::test]
    async fn get_planet() {
        let api = Contracts::new_xdai_readonly();
        let home = from_planet();
        let planet = api.planet(home).await.unwrap();
        dbg!(&planet);
    }

    #[tokio::test]
    // checks that we can submit a move
    async fn solidity_move() {
        let prover: MoveProver = circuit_prover(Some("round3-data/move.zkey"), "move");
        let move_args = MoveArgs {
            // values are multiplied x1000
            population: 95_000.into(),
            silver: 0.into(),
            moved_artifact: None,
        };
        let network = Network::Xdai;

        let provider = Provider::try_from("https://dai.poa.network").unwrap();
        use std::str::FromStr;
        let signer = LocalWallet::from_str(
            "a046a5b763923d437855a6fe64962569c9a378efba5c84920212c4b6ae270df5",
        )
        .unwrap();
        let provider = SignerMiddleware::new(provider, signer);
        let provider = Arc::new(provider);
        let api = Contracts::new(provider, network);

        let from = from_planet();
        let planet = api.planet(from.clone()).await.unwrap();
        let planet_init = api.planet_initialized(from.hash).await.unwrap();
        dbg!(&planet);
        dbg!(&planet_init);

        let to = to_planet();
        let planet = api.planet(to.clone()).await.unwrap();
        dbg!(&planet);

        let (proof, inputs) = prover.prove(&from, &to).unwrap();

        let vk = prover.0.params.vk;
        let pvk = ark_groth16::prepare_verifying_key(&vk);
        let verified = ark_groth16::verify_proof(&pvk, &proof, &inputs).unwrap();
        assert!(verified);
        dbg!("OKK");

        let call = api
            .move_(&from, &to, &move_args, proof)
            .gas_price(ethers::utils::parse_units("1", 9).unwrap());
        let res = call.send().await.unwrap();
        // dbg!(*res);

        let receipt = res.await.unwrap().unwrap();
        dbg!(&receipt);
    }
}
