// copied from https://github.com/projectsophon/darkforest-rs/blob/3d47064b92068e19e4ded75acc3b846064fda8a6/darkforest/src/explorers.rs
#![allow(unused)]
use super::{ChunkFootprint, Coords, Direction};

/// A Directional Iterator of ChunkFootprint
pub struct DirectionalExplorer {
    chunk_side_length: u16,
    from_chunk: ChunkFootprint,
    current_chunk: ChunkFootprint,
}

pub enum MixedDirection {
    Single(Direction),
    Multi((Direction, Direction)),
}

impl From<Direction> for MixedDirection {
    fn from(src: Direction) -> MixedDirection {
        MixedDirection::Single(src)
    }
}

impl From<(Direction, Direction)> for MixedDirection {
    fn from(src: (Direction, Direction)) -> MixedDirection {
        MixedDirection::Multi(src)
    }
}

impl DirectionalExplorer {
    pub fn new<D: Into<MixedDirection>>(
        center: Coords,
        chunk_side_length: u16,
        direction: D,
    ) -> Self {
        unimplemented!()
    }
}

impl Iterator for DirectionalExplorer {
    type Item = ChunkFootprint;
    fn next(&mut self) -> Option<ChunkFootprint> {
        unimplemented!()
    }
}
