pub mod prover;
pub use prover::init::InitProver;
pub use prover::mover::MoveProver;
pub use prover::CircuitProver;
pub use prover::Prover;

pub mod contracts;
pub use contracts::Contracts;

use ethers::types::{Address, TxHash};
use reqwest::{Client, Response};

const DF_REGISTER_API: &str = "https://api.zkga.me/whitelist/register";

use df_types::planet::Coords;

// pls dont attack
pub const HOME: Coords = Coords {
    x: -305389,
    y: 188332,
};

pub const TARGET: Coords = Coords {
    x: -305468,
    y: 188340,
};

/// Registers your Address to the provided Whitelist Key by making a POST request
/// to the Dark Forest API. Returns the submitted transaction hash which can be used
/// to track when registration is done.
pub async fn register(key: &str, address: Address) -> Result<TxHash, reqwest::Error> {
    let client = Client::new();
    let res: Response = client
        .post(DF_REGISTER_API)
        .json(&serde_json::json!({
            "key": key,
            "address": address,
        }))
        .send()
        .await?;

    #[derive(serde::Deserialize)]
    struct RegisterResponse {
        #[serde(rename = "txHash")]
        tx_hash: TxHash,
    }
    let data: RegisterResponse = res.json().await?;

    Ok(data.tx_hash)
}

#[cfg(test)]
mod tests {
    use super::*;
    use df_types::planet::{Coords, PlanetId, PlanetLocation};
    use std::convert::TryFrom;
    use std::path::PathBuf;

    pub fn from_planet() -> PlanetLocation {
        let coords = Coords::from(HOME);
        let hash = PlanetId::try_from(&coords).unwrap();
        PlanetLocation {
            coords,
            hash,
            // ADD PERLIN GENERATION FUNCTION
            perlin: 19,
            biomebase: 16,
        }
    }

    pub fn to_planet() -> PlanetLocation {
        let coords = Coords::from(TARGET);
        let hash = PlanetId::try_from(&coords).unwrap();
        let to = PlanetLocation {
            coords,
            hash,
            perlin: 19,
            biomebase: 16,
        };
        to
    }

    pub fn root_path(p: &str) -> PathBuf {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push(p);
        path
    }
}
