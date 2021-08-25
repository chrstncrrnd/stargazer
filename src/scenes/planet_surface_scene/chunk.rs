use bracket_noise::prelude::FastNoise;
use macroquad::prelude::*;

use crate::nodes::block::block::Blocks::{Dirt, Grass};
use crate::nodes::block::block::{Block, Blocks, Facing};
use crate::resources::BlockResources;

pub struct Chunk {
    pub position: Vec2,
    pub blocks: Vec<Block>,
}


impl Chunk {
    pub fn load(position: Vec2) -> Self {
        Chunk {
            position,
            blocks: Vec::new(),
        }
    }

    fn get_facing_direction(position: Vec2) -> Facing{
        let mut num = position.x + position.y;

        num = num % 10.;
        
        if num < 2.{
            Facing::Left
        }else if num < 4.{
            Facing::Right
        }else if num < 6.{
            Facing::Up
        } else{
            Facing::Down
        }

    }

    pub fn render(
        &mut self,
        blocksize: usize,
        block_resources: &BlockResources,
        noise: &FastNoise,
    ) {
        //populate the blocks
        if self.blocks.is_empty() {
            for x in (0..blocksize * 4).step_by(blocksize) {
                for y in (0..blocksize * 4).step_by(blocksize) {
                    let current_noise = noise.get_noise(
                        self.position.x + ((x) as f32) / 1000.,
                        self.position.y + ((y) as f32) / 1000.,
                    );
                    let mut block = Block::new(
                        vec2(self.position.x + x as f32, self.position.y + y as f32),
                        Blocks::Error,
                        blocksize,
                        Self::get_facing_direction(self.position)
                    );
                    if current_noise < 0.0 {
                        block.block_type = Dirt;
                    } else {
                        block.block_type = Grass;
                    }
                    self.blocks.push(block);
                }
            }
        }

        for block in self.blocks.iter() {
            block.render(&block_resources);
        }
    }
}
