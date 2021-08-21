use bracket_noise::prelude::*;
use macroquad::math::{Vec2, vec2};

use crate::game_views::planet_view::block::block;
use macroquad::prelude::Texture2D;
use crate::resources::BlockResources;
use crate::game_views::planet_view::block::block::{Block, Blocks};

pub struct PlanetSurface {
    pub render_distance: Vec2,
    pub origin: Vec2,
    pub noise: FastNoise
}

impl PlanetSurface {
    pub fn render(&mut self, block_resources: &BlockResources) {
        let block_size: f32 = 70.;
        for x_pos in (0..self.render_distance.x as i32).step_by(block_size as usize){
            for y_pos in (0..self.render_distance.y as i32).step_by(block_size as usize){
                let current_noise = self.noise.get_noise(x_pos as f32 / 2000., y_pos as f32 / 1600.);
                if current_noise < 0.0 {
                    let some_block = Block::new(vec2(x_pos as f32, y_pos as f32), Blocks::Dirt);
                    some_block.render(block_resources);
                }else{
                    let some_block = Block::new(vec2(x_pos as f32, y_pos as f32), Blocks::Grass);
                    some_block.render(block_resources);
                }
            }
        }
    }
}
