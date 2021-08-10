use itertools::iproduct;

use crate::{
    map::ChunkFootprint,
    planet::Coords,
    planet::{PlanetId, PlanetLocation},
};

mod spiral;
pub use spiral::SpiralExplorer;
use std::convert::TryFrom;

pub fn explore_spiral(center: Coords, chunk_size: u16) -> impl Iterator<Item = PlanetLocation> {
    explore(SpiralExplorer::new(center, chunk_size))
}

// how big of chunks, client default is 16, remote miner default is 256
pub fn explore<I: IntoIterator<Item = ChunkFootprint>>(
    explorer: I,
) -> impl Iterator<Item = PlanetLocation> {
    explorer
        .into_iter()
        .map(reveal_chunk)
        .flatten()
}

fn reveal_chunk(chunk: ChunkFootprint) -> impl Iterator<Item = PlanetLocation> {
    let ChunkFootprint {
        side_length: size,
        bottom_left: Coords { x, y },
    } = chunk;
    let size = size as i64;

    iproduct!(x..(x + size), y..(y + size))
        .filter_map(|(xi, yi)| {
            let coords = Coords { x: xi, y: yi };
            PlanetId::try_from(&coords).ok().map(|hash| (coords, hash))
        })
        .map(|(coords, hash)| PlanetLocation {
            coords,
            hash,
            perlin: 12,
            biomebase: 0,
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_find_planet() {
        let center = Coords { x: 0, y: 0 };
        let iter = explore_spiral(center, 1024 as u16);
        let items = iter.take(3).collect::<Vec<_>>();
        dbg!(&items);
    }
}
