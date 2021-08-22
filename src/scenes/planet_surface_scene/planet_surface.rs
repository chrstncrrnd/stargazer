use bracket_noise::prelude::*;
use macroquad::math::{Rect, Vec2, vec2};
use macroquad::prelude::miniquad::KeyCode::D;
use macroquad::prelude::Texture2D;
use macroquad::window::{screen_height, screen_width};

use crate::nodes::block::block::{Block, Blocks};
use crate::nodes::player::Player;
use crate::resources::BlockResources;
use crate::scenes::planet_surface_scene::chunk::Chunk;

pub struct PlanetSurface {
    pub noise: FastNoise
}

impl PlanetSurface {
    pub fn render(&mut self, block_resources: &BlockResources, player: &Player) {
        let block_size: usize = 50;

        let render_area = Rect{
            x: player.position.x - (screen_width() / 2.),
            y: player.position.y - (screen_height() /2.),
            w: screen_width(),
            h: screen_height()
        };

        for x in render_area.x as i32..(render_area.x + render_area.w) as i32 {
            for y in render_area.y as i32..(render_area.y + render_area.h) as i32{
                if x % (block_size*4) as i32 == 0 && y % (block_size*4) as i32 ==0 {
                    let chunk = Chunk::load(vec2(x as f32, y as f32));
                    chunk.render(block_size, &block_resources, &self.noise);
                    drop(chunk);
                }
            }
        }
    }
}
