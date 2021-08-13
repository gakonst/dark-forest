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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
#[serde(rename_all = "camelCase")]
pub struct Chunk {
    pub chunk_footprint: ChunkFootprint,
    pub perlin: f64,
    pub planet_locations: Vec<PlanetLocation>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
#[serde(rename_all = "camelCase")]
pub struct ChunkFootprint {
    pub bottom_left: Coords,
    pub side_length: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::root_path;

    #[test]
    fn serde_map() {
        let path = root_path("../../test-vectors/large-map.json");
        let map = std::fs::read_to_string(path).unwrap();
        let map: Map = serde_json::from_str(&map).unwrap();
        println!("{:#?}", map);
    }
}
