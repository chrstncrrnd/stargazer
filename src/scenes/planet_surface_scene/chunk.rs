use bracket_noise::prelude::FastNoise;
use macroquad::prelude::*;

use crate::nodes::block::block::{Block, Blocks};
use crate::nodes::block::block::Blocks::{Dirt, Grass};
use crate::resources::BlockResources;
use bracket_noise::prelude::NoiseType::{Perlin, PerlinFractal};

pub struct Chunk{
    pub position: Vec2
}

impl Chunk{
    pub fn load(position: Vec2) -> Self{
        Chunk{
            position,
        }
    }

    //TODO: make this faster omg its soooooooooo slow holy fuck
    pub fn render(&self, blocksize: usize, block_resources: &BlockResources, noise: &FastNoise){
        for x in (0..blocksize*4).step_by(blocksize){
            for y in (0..blocksize*4).step_by(blocksize){
                let current_noise = noise.get_noise(self.position.x + ((x) as f32)/1000.,self.position.y + ((y) as f32)/1000.);
                let mut block = Block::new(vec2(self.position.x + x as f32, self.position.y + y as f32), Blocks::Error);
                if current_noise < 0.0{
                    block.block_type = Dirt;
                }
                else{
                    block.block_type = Grass;
                }
                block.render(block_resources);
            }
        }
    }

}

