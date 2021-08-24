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

struct RectI{
    x: i32,
    y: i32,
    w: i32,
    h: i32
}

struct Vec2I{
    x: i32,
    y: i32
}

impl Vec2I{
    pub fn from_vec2(vec2: Vec2) -> Self{
        Vec2I{
            x: vec2.x as i32,
            y: vec2.y as i32
        }
    }
    pub fn to_vec2(x: i32, y: i32) -> Vec2{
        vec2(x as f32, y as f32)
    }


}

impl PlanetSurface {

    fn is_in_render_area(render_area: &RectI, position: Vec2I) -> bool {
        if position.x > render_area.x && position.x < render_area.x + render_area.w {
            if position.y > render_area.y && position.y < render_area.y + render_area.h{
                return true;
            }
        }
        return false;
    }

    fn round_to_nearest(initial_number: f32, multiple_of: i32) -> i32{
        if initial_number as i32 % multiple_of == 0{
            return initial_number as i32;
        }

        for i in initial_number as i32.. {
            if i % multiple_of == 0{
                return i;
            }
        }
        initial_number as i32
    }

    fn spawn_chunk(&mut self, x: i32, y: i32){
        self.chunks.push(Chunk::load(vec2(x as f32, y as f32)));
    }

    pub fn render(&mut self, block_resources: &BlockResources, player: &Player) {
        let mut chunk_exists: bool = false;
        let block_size: i32 = 70;

        let render_area = Rect {
            x: player.position.x - (screen_width() / 2.) - (block_size*4*2) as f32,
            y: player.position.y - (screen_height() / 2.) - (block_size*4*2) as f32,
            w: screen_width() + (block_size*4*2) as f32,
            h: screen_height() + (block_size*4*2) as f32,
        };

        let spawnable_area = RectI{
            x: PlanetSurface::round_to_nearest(render_area.x, block_size * 4) as i32,
            y: PlanetSurface::round_to_nearest(render_area.y, block_size * 4) as i32,
            w: PlanetSurface::round_to_nearest(render_area.w, block_size * 4) as i32,
            h: PlanetSurface::round_to_nearest(render_area.h, block_size * 4) as i32,
        };

        if self.chunks.is_empty() {
            for x in (spawnable_area.x..spawnable_area.x + spawnable_area.w).step_by((block_size*4) as usize) {
                for y in (spawnable_area.y..spawnable_area.y + spawnable_area.h).step_by((block_size*4) as usize) {
                    self.spawn_chunk(x,y);
                }
            }
        }

        for x in (spawnable_area.x..spawnable_area.x + spawnable_area.w).step_by((block_size*4) as usize) {
            for y in (spawnable_area.y..spawnable_area.y + spawnable_area.h).step_by((block_size*4) as usize) {
                chunk_exists = false;
                for chunk in self.chunks.iter_mut(){
                    if chunk.position == Vec2I::to_vec2(x, y){
                        chunk_exists = true;
                    }
                }
                if !chunk_exists{
                    self.spawn_chunk(x, y);
                }
            }
        }


        self.chunks.retain(|chunk| PlanetSurface::is_in_render_area(&spawnable_area, Vec2I::from_vec2(chunk.position)));

        for chunk in self.chunks.iter_mut() {
            chunk.render(block_size as usize, block_resources, &self.noise);
        }

    }
}
