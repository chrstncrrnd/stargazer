use crate::world::terrain::BlockType;
use macroquad::math::Vec2;

pub struct Block {
    pub position: Vec2,
    pub block_type: BlockType,
}

impl Block {
    pub fn new(position: Vec2, block_type: BlockType) -> Self {
        Self {
            position,
            block_type,
        }
    }
}
