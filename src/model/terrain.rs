use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum BlockType {
    Air,
    Dirt,
    Stone,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Terrain {
    pub blocks: Vec<BlockType>,
    pub side: i32,
}

impl Terrain {
    pub fn new(side: i32) -> Terrain {
        Terrain {
            blocks: vec![BlockType::Air; (side * side * side) as usize],
            side,
        }
    }

    pub fn block(&self, i: IVec3) -> BlockType {
        self.blocks[((i.x) + (i.z) * self.side + (i.y) * self.side * self.side) as usize]
    }
}
