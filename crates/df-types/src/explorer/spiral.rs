// copied from https://github.com/projectsophon/darkforest-rs/blob/3d47064b92068e19e4ded75acc3b846064fda8a6/darkforest/src/explorers.rs
use super::{ChunkFootprint, Coords};

/// A Spiral Iterator of ChunkFootprint
pub struct SpiralExplorer {
    chunk_side_length: u16,
    from_chunk: ChunkFootprint,
    current_chunk: ChunkFootprint,
}

impl SpiralExplorer {
    pub fn new(center: Coords, chunk_side_length: u16) -> Self {
        let length = i64::from(chunk_side_length);

        let bottom_left_x = (center.x / length) * length;
        let bottom_left_y = (center.y / length) * length;
        let bottom_left = Coords {
            x: bottom_left_x,
            y: bottom_left_y,
        };

        let from_chunk = ChunkFootprint {
            bottom_left,
            side_length: length as u64,
        };

        Self {
            chunk_side_length,
            from_chunk: from_chunk.clone(),
            current_chunk: from_chunk,
        }
    }
}

impl Iterator for SpiralExplorer {
    type Item = ChunkFootprint;
    fn next(&mut self) -> Option<ChunkFootprint> {
        let Coords {
            x: home_x,
            y: home_y,
        } = self.from_chunk.bottom_left;
        let Coords {
            x: curr_x,
            y: curr_y,
        } = self.current_chunk.bottom_left;

        let mut next_bottom_left = self.current_chunk.bottom_left.clone();

        let length = i64::from(self.chunk_side_length);

        if curr_x == home_x && curr_y == home_y {
            next_bottom_left.y = home_y + length;
        } else if curr_y - curr_x > home_y - home_x && curr_y + curr_x >= home_x + home_y {
            if curr_y + curr_x == home_x + home_y {
                // break the circle
                next_bottom_left.y = curr_y + length;
            } else {
                next_bottom_left.x = curr_x + length;
            }
        } else if curr_x + curr_y > home_x + home_y && curr_y - curr_x <= home_y - home_x {
            next_bottom_left.y = curr_y - length;
        } else if curr_x + curr_y <= home_x + home_y && curr_y - curr_x < home_y - home_x {
            next_bottom_left.x = curr_x - length;
        } else {
            // if (curr_x + curr_y < home_x + home_y && curr_y - curr_x >= home_y - home_x)
            next_bottom_left.y = curr_y + length;
        }

        self.current_chunk = ChunkFootprint {
            bottom_left: next_bottom_left,
            side_length: length as u64,
        };
        Some(self.current_chunk.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sixteen_iter() {
        let center = Coords { x: 0, y: 0 };
        let chunk_side_length = 16 as u64;
        let mut explorer = SpiralExplorer::new(center, chunk_side_length as u16);

        assert_eq!(
            explorer.next(),
            Some(ChunkFootprint {
                bottom_left: Coords {
                    x: 0,
                    y: chunk_side_length as i64,
                },
                side_length: chunk_side_length
            })
        );

        assert_eq!(
            explorer.next(),
            Some(ChunkFootprint {
                bottom_left: Coords {
                    x: chunk_side_length as i64,
                    y: chunk_side_length as i64,
                },
                side_length: chunk_side_length
            })
        );

        assert_eq!(
            explorer.next(),
            Some(ChunkFootprint {
                bottom_left: Coords {
                    x: chunk_side_length as i64,
                    y: 0,
                },
                side_length: chunk_side_length
            })
        );

        assert_eq!(
            explorer.next(),
            Some(ChunkFootprint {
                bottom_left: Coords {
                    x: chunk_side_length as i64,
                    y: -(chunk_side_length as i64),
                },
                side_length: chunk_side_length
            })
        );

        assert_eq!(
            explorer.next(),
            Some(ChunkFootprint {
                bottom_left: Coords {
                    x: 0,
                    y: -(chunk_side_length as i64),
                },
                side_length: chunk_side_length
            })
        );
    }

    #[test]
    fn thirtytwo_iter() {
        let center = Coords { x: 0, y: 0 };
        let chunk_side_length = 32 as u64;
        let mut explorer = SpiralExplorer::new(center, chunk_side_length as u16);

        assert_eq!(
            explorer.next(),
            Some(ChunkFootprint {
                bottom_left: Coords {
                    x: 0,
                    y: chunk_side_length as i64,
                },
                side_length: chunk_side_length
            })
        );

        assert_eq!(
            explorer.next(),
            Some(ChunkFootprint {
                bottom_left: Coords {
                    x: chunk_side_length as i64,
                    y: chunk_side_length as i64,
                },
                side_length: chunk_side_length
            })
        );

        assert_eq!(
            explorer.next(),
            Some(ChunkFootprint {
                bottom_left: Coords {
                    x: chunk_side_length as i64,
                    y: 0,
                },
                side_length: chunk_side_length
            })
        );

        assert_eq!(
            explorer.next(),
            Some(ChunkFootprint {
                bottom_left: Coords {
                    x: chunk_side_length as i64,
                    y: -(chunk_side_length as i64),
                },
                side_length: chunk_side_length
            })
        );

        assert_eq!(
            explorer.next(),
            Some(ChunkFootprint {
                bottom_left: Coords {
                    x: 0,
                    y: -(chunk_side_length as i64),
                },
                side_length: chunk_side_length
            })
        );
    }
}
