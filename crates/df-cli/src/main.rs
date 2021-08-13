use color_eyre::eyre::Result;
use ethers::{
    prelude::{LocalWallet, SignerMiddleware},
    providers::{Http, Provider},
    utils::parse_units,
};
use std::{convert::TryFrom, path::PathBuf, sync::Arc, time::Duration};
use structopt::StructOpt;

use dark_forest::{
    contracts::Network,
    engine::{contracts::MoveArgs, CircuitProver, Contracts, MoveProver},
    types::planet::{Coords, PlanetLocation},
};
use std::str::FromStr;

#[derive(Debug, StructOpt)]
#[structopt(about = "df cli with all commands")]
enum Opts {
    Map(MapOpts),
    Move(MoveOpts),
}

#[derive(Debug, StructOpt)]
struct MapOpts {
    // https://www.xdaichain.com/for-developers/developer-resources#json-rpc-endpoints
    #[structopt(short, long, default_value = "https://dai.poa.network")]
    url: String,
    map: PathBuf,
}

use std::error::Error;
fn parse_coords(s: &str) -> Result<Coords, Box<dyn Error>> {
    let mut s = s.split(',').map(|x| i64::from_str(x)).flatten();
    let x = s.next().unwrap();
    let y = s.next().unwrap();
    Ok(Coords { x, y })
}

#[derive(Debug, StructOpt)]
struct MoveOpts {
    // https://www.xdaichain.com/for-developers/developer-resources#json-rpc-endpoints
    #[structopt(short, long, default_value = "https://dai.poa.network")]
    url: String,
    #[structopt(short, long)]
    private_key: String,

    #[structopt(short, long, parse(try_from_str = parse_coords))]
    from: Coords,

    #[structopt(short, long, parse(try_from_str = parse_coords))]
    to: Coords,

    #[structopt(short, long)]
    wasm: PathBuf,

    #[structopt(short, long)]
    r1cs: PathBuf,

    #[structopt(short, long)]
    zkey: PathBuf,

    #[structopt(short, long)]
    population: u64,

    #[structopt(short, long)]
    silver: u64,

    #[structopt(short, long)]
    artifact_id: Option<u64>,
}

mod map;
use map::print_map;

#[tokio::main]
async fn main() -> Result<()> {
    let opts: Opts = Opts::from_args();

    match opts {
        Opts::Map(MapOpts { url, map }) => {
            let provider = Provider::<Http>::try_from(url.as_str())?;
            let provider = Arc::new(provider);
            let contracts = Contracts::new(provider, Network::Xdai);
            print_map(map, contracts).await?;
        }
        Opts::Move(opts) => {
            let provider =
                Provider::<Http>::try_from(opts.url.as_str())?.interval(Duration::from_secs(2));
            let signer = LocalWallet::from_str(&opts.private_key)?;
            let provider = SignerMiddleware::new(provider, signer);
            let provider = Arc::new(provider);
            let contracts = Contracts::new(provider, Network::Xdai);

            let prover: MoveProver =
                CircuitProver::new_path(opts.zkey, opts.wasm, opts.r1cs)?.into();
            let from = PlanetLocation::try_from(opts.from)?;
            let to = PlanetLocation::try_from(opts.to)?;
            let (proof, _inputs) = prover.prove(&from, &to)?;

            let args = MoveArgs {
                population: opts.population.into(),
                silver: opts.silver.into(),
                moved_artifact: opts.artifact_id.map(ethers::types::U256::from),
            };

            let planet_from = contracts.planet(from.clone()).await?;
            let planet_to = contracts.planet(to.clone()).await?;

            println!(
                "Sending {:?} forces from {:?} (total pop: {:?}), to {:?} (total pop: {:?}).",
                args.population,
                from.coords,
                planet_from.planet.population,
                to.coords,
                planet_to.planet.population
            );

            let call = contracts
                .move_(&from, &to, &args, proof)
                .gas_price(parse_units("2", 9).unwrap());

            let pending_tx = match call.send().await {
                Ok(pending) => pending,
                Err(err) => {
                    let err = err.to_string();
                    let err = err.split("Reverted 0x").collect::<Vec<&str>>()[1];
                    let err = err.split(',').collect::<Vec<&str>>()[0];
                    let err = hex::decode(err).unwrap();
                    let err = std::str::from_utf8(&err).unwrap();
                    println!("Reverted with: {}", err);
                    return Ok(());
                }
            };

            println!(
                "Sent transaction https://blockscout.com/xdai/mainnet/tx/{:?}",
                *pending_tx
            );

            let confirmed = pending_tx.await?;
            println!("Confirmed {:?}", confirmed);

            let planet_from = contracts.planet(from.clone()).await?;
            let planet_to = contracts.planet(to.clone()).await?;

            println!(
                "from total pop after: {:?}), to total pop after {:?}.",
                planet_from.planet.population, planet_to.planet.population
            );
        }
    };

    Ok(())
}
