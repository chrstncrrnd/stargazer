use bracket_noise::prelude::*;
use macroquad::math::{Vec2, vec2, Rect};
use macroquad::prelude::Texture2D;
use macroquad::window::{screen_height, screen_width};

use crate::nodes::block::block::{Block, Blocks};
use crate::nodes::player::Player;
use crate::resources::BlockResources;

pub struct PlanetSurface {
    pub noise: FastNoise
}

impl PlanetSurface {
    pub fn render(&mut self, block_resources: &BlockResources, player: &Player) {
        let block_size: f32 = 50.;

        let render_area = Rect{
            x: player.position.x - screen_width() / 2. - 100.,
            y: player.position.y - screen_height() /2. - 100.,
            w: screen_width() + 100.,
            h: screen_height() + 100.
        };

        //for block positions in the thing with the blocks:
        for x_pos in (-10000..10000).step_by(block_size as usize){
            for y_pos in (-10000..10000).step_by(block_size as usize){
                let x = render_area.x as i32;
                let y = render_area.y as i32;
                let w = render_area.w as i32;
                let h = render_area.h as i32;
                if x_pos > x && x_pos < x + w {
                    if y_pos > y && y_pos < y + h{
                        let pos_x: f32 = x_pos as f32;
                        let pos_y: f32 = y_pos as f32;
                        let current_noise = self.noise.get_noise(pos_x / 1000., pos_y / 1000.);
                        let mut block = Block::new(vec2(pos_x, pos_y), Blocks::Error);

                        if current_noise < 0.0 {
                            block.block_type = Blocks::Grass;
                            block.render(block_resources);
                        } else {
                            block.block_type = Blocks::Dirt;
                            block.render(block_resources);
                        }


                    }
                }


            }

        }



        // let block_size: f32 = 70.;
        // for x_pos in (0..self.render_distance.x as i32).step_by(block_size as usize){
        //     for y_pos in (0..self.render_distance.y as i32).step_by(block_size as usize){
        //         let current_noise = self.noise.get_noise(x_pos as f32 / 2000., y_pos as f32 / 1600.);
        //         let mut block = Block::new(vec2(x_pos as f32, y_pos as f32), Blocks::Error);
        //         if current_noise < 0.0 {
        //             block.block_type = Blocks::Sand;
        //             block.render(block_resources);
        //         }else if current_noise < 0.5{
        //             block.block_type = Blocks::Dirt;
        //             block.render(block_resources);
        //         }
        //         else{
        //             block.block_type = Blocks::Sand;
        //             block.render(block_resources);
        //         }
        //     }
        // }
    }
}
