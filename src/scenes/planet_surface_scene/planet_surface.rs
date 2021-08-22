use bracket_noise::prelude::*;
use macroquad::math::{vec2, Rect, Vec2};
use macroquad::window::{screen_height, screen_width};

use crate::nodes::player::Player;
use crate::resources::BlockResources;
use crate::scenes::planet_surface_scene::chunk::Chunk;

pub struct PlanetSurface {
    pub noise: FastNoise,
    pub chunks: Vec<Chunk>,
}

impl PlanetSurface {
    fn is_in_render_area(render_area: &Rect, position: Vec2) -> bool {
        if position.x > render_area.x && position.x < render_area.x + render_area.w {
            if position.y > render_area.y && position.y < render_area.y + render_area.h {
                return true;
            }
        }
        return false;
    }

    //TODO write a function that checks teh render area and see if all of the possibe locations are filled, if they arent then push chunks there.

    pub fn render(&mut self, block_resources: &BlockResources, player: &Player) {
        let mut chunks_to_remove: Vec<usize> = Vec::new();
        let block_size: usize = 50;

        let render_area = Rect {
            x: player.position.x - (screen_width() / 2.) - 240.,
            y: player.position.y - (screen_height() / 2.) - 240.,
            w: screen_width() + 240.,
            h: screen_height() +240.,
        };

        //repopulate all of the chunks if they are empty
        if self.chunks.is_empty() {
            for x in render_area.x as i32..(render_area.x + render_area.w) as i32 {
                for y in render_area.y as i32..(render_area.y + render_area.h) as i32 {
                    if x % (block_size * 4) as i32 == 0 && y % (block_size * 4) as i32 == 0 {
                        let chunk = Chunk::load(vec2(x as f32, y as f32));
                        self.chunks.push(chunk);
                    }
                }
            }
        }

        self.render_area(&render_area, block_size);
        //remove the chunks that arent in render distance
        self.chunks
            .retain(|chunk| PlanetSurface::is_in_render_area(&render_area, chunk.position));
        //render all of the chunks
        for chunk in self.chunks.iter_mut() {
            chunk.render(block_size, block_resources, &self.noise)
        }
    }
}
