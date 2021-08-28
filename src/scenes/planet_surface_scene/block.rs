use crate::world_generation::terrain::BlockType;
use macroquad::math::Vec2;

pub struct Block {
    pub(crate) position: Vec2,
    pub(crate) block_type: BlockType,
}
