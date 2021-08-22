use bracket_noise::prelude::FastNoise;
use macroquad::prelude::*;

use crate::nodes::block::block::Blocks::{Dirt, Grass};
use crate::nodes::block::block::{Block, Blocks};
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

    pub fn render(
        &mut self,
        blocksize: usize,
        block_resources: &BlockResources,
        noise: &FastNoise,
    ) {
        if self.blocks.is_empty() {
            for x in (0..blocksize * 4).step_by(blocksize) {
                for y in (0..blocksize * 4).step_by(blocksize) {
                    let mut block = Block::new(
                        vec2(self.position.x + x as f32, self.position.y + y as f32),
                        Blocks::Error,
                    );
                    let current_noise = noise.get_noise(
                        self.position.x + ((x) as f32) / 1000.,
                        self.position.y + ((y) as f32) / 1000.,
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
