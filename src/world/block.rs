use crate::world::terrain::BlockType;
use macroquad::math::Vec2;

pub struct Block {
    pub position: Vec2,
    pub block_type: BlockType,
    pub shadow: u8,
    pub height: i32
}

impl Block {
    pub fn new(position: Vec2, block_type: BlockType, shadow: u8) -> Self {
        Self {
            position,
            block_type,
            shadow,
            height: 0
        }
    }

    pub fn new_height(position: Vec2, block_type: BlockType, shadow: u8, height: i32) -> Self {
        Self {
            position,
            block_type,
            shadow,
            height
        }
    }
    
}
