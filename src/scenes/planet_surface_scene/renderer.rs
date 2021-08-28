use crate::nodes::player::Player;
use crate::resources::BlockResources;
use crate::scenes::planet_surface_scene::block::Block;
use crate::world_generation::terrain::BlockType;
use crate::world_generation::terrain::TerrainGenerator;
use macroquad::prelude::*;

pub struct Renderer {
    pub blocks: Vec<Block>,
    pub blocksize: i32,
    pub terrain_generator: Box<dyn TerrainGenerator>,
    pub runner_step: u8,
}

impl Renderer {
    pub fn init(&mut self) {
        self.terrain_generator.init();
    }
    fn round_to_nearest(initial_number: f32, multiple_of: i32) -> i32 {
        if initial_number as i32 % multiple_of == 0 {
            return initial_number as i32;
        }

        for i in initial_number as i32.. {
            if i % multiple_of == 0 {
                return i;
            }
        }
        initial_number as i32
    }

    fn is_in_render_area(render_area: &Rect, position: Vec2) -> bool {
        if position.x > render_area.x && position.x < render_area.x + render_area.w {
            if position.y > render_area.y && position.y < render_area.y + render_area.h{
                return true;
            }
        }
        return false;
    }



    pub fn render(&mut self, player: &Player, textures: &BlockResources) {
        //render the blocks
        let mut block_exists: bool;

        let render_area = Rect {
            x: player.position.x - (screen_width() / 2.) - (100) as f32,
            y: player.position.y - (screen_height() / 2.) - (100) as f32,
            w: screen_width() + (200) as f32,
            h: screen_height() + (200) as f32,
        };

        let spawnable_area = Rect {
            x: Self::round_to_nearest(render_area.x, self.blocksize) as f32,
            y: Self::round_to_nearest(render_area.y, self.blocksize) as f32,
            w: Self::round_to_nearest(render_area.w, self.blocksize) as f32,
            h: Self::round_to_nearest(render_area.h, self.blocksize) as f32,
        };

        if self.blocks.is_empty() {
            for x in (spawnable_area.x as i32..(spawnable_area.x + spawnable_area.w) as i32)
                .step_by((self.blocksize) as usize)
            {
                for y in (spawnable_area.y as i32..(spawnable_area.y + spawnable_area.h) as i32)
                    .step_by((self.blocksize) as usize)
                {
                    self.blocks.push(Block {
                        position: vec2(x as f32, y as f32),
                        block_type: self.terrain_generator.get_block(vec2(x as f32, y as f32)),
                    })
                }
            }
        }
        if self.runner_step == 5 {
            for x in (spawnable_area.x as i32..(spawnable_area.x + spawnable_area.w) as i32)
                .step_by((self.blocksize) as usize)
            {
                for y in (spawnable_area.y as i32..(spawnable_area.y + spawnable_area.h) as i32)
                    .step_by((self.blocksize) as usize)
                {
                    block_exists = false;
                    for block in self.blocks.iter() {
                        if block.position == vec2(x as f32, y as f32) {
                            block_exists = true;
                            break
                        }
                    }
                    if !block_exists {
                        self.blocks.push(Block {
                            position: vec2(x as f32, y as f32),
                            block_type: self.terrain_generator.get_block(vec2(x as f32, y as f32))
                        })
                    }
                }
            }
            self.runner_step = 0;
        }
        self.runner_step += 1;

        self.blocks.retain(|block| Renderer::is_in_render_area(&render_area, block.position));

        for block in self.blocks.iter() {
            draw_texture_ex(
                match block.block_type {
                    BlockType::Grass => textures.grass,
                    BlockType::Dirt => textures.dirt,
                    BlockType::Error => textures.error,
                    BlockType::Sand => textures.sand,
                    BlockType::Mud => textures.mud,
                    BlockType::Water => textures.water
                },
                block.position.x,
                block.position.y,
                WHITE,
                DrawTextureParams {
                    dest_size: Option::from(vec2(self.blocksize as f32, self.blocksize as f32)),
                    ..Default::default()
                },
            )
        }

    }
}
