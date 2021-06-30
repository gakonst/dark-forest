//! Utility for reading the map.json exported by the [Maps DF
//! plugin](https://github.com/darkforest-eth/plugins/tree/master/content/utilities/map-export)
use super::planet::{Coords, PlanetLocation};
use serde::{Deserialize, Serialize};

/// The map as exported from the map export plugin
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Map(Vec<Chunk>);

impl Map {
    /// Filters the map for planet locations
    pub fn planets(&self) -> Vec<PlanetLocation> {
        self.0
            .iter()
            .map(|chunk| chunk.planet_locations.clone())
            .flatten()
            .collect()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Chunk {
    chunk_footprint: ChunkFootprint,
    perlin: f64,
    planet_locations: Vec<PlanetLocation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChunkFootprint {
    bottom_left: Coords,
    side_length: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serde_map() {
        let map = std::fs::read_to_string("./test-vectors/large-map.json").unwrap();
        let map: Map = serde_json::from_str(&map).unwrap();
        println!("{:#?}", map);
    }
}
