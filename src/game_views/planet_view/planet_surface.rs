use bracket_noise::prelude::*;
use macroquad::math::Vec2;

use crate::game_views::planet_view::block::block;
use macroquad::prelude::Texture2D;

pub struct PlanetSurface {
    pub render_distance: Vec2,
    pub origin: Vec2,
    pub block: block::Block,
}

impl PlanetSurface {
    pub fn render(&mut self) {
        for x in (0..self.render_distance.x as i32).step_by(self.block.size.x as usize) {
            for y in (0..self.render_distance.y as i32).step_by(self.block.size.y as usize) {
                self.block.position.x = x as f32;
                self.block.position.y = y as f32;
                self.block.render();
            }
        }
    }
}
