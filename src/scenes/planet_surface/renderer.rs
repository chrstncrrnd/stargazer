use crate::nodes::player::Player;
use crate::resources::BlockResources;
use crate::world::block::Block;
use crate::world::terrain::BlockType;
use crate::world::terrain::TerrainGenerator;
use macroquad::prelude::*;
use std::f32::consts::FRAC_2_PI;

const BLOCK_SIZE: usize = 50;

pub struct Renderer {
    blocks: Vec<Block>,
    terrain_generator: Box<dyn TerrainGenerator>,
    prev_render_area: Rect,
}

impl Renderer {
    pub fn new(terrain_generator: Box<dyn TerrainGenerator>) -> Self {
        Self {
            blocks: Vec::new(),
            terrain_generator,
            prev_render_area: Rect::new(0., 0., 0., 0.),
        }
    }

    fn add_block_if_not_there(&mut self, block1: Block) {
        for block in self.blocks.iter() {
            if block.position == block1.position {
                return;
            }
        }
        self.blocks.push(block1);
    }

    #[inline]
    fn i32_vec2(x: i32, y: i32) -> Vec2 {
        return vec2(x as f32, y as f32);
    }

    #[inline]
    fn is_in_render_area(render_area: &Rect, position: Vec2) -> bool {
        position.x > render_area.x && position.x < render_area.x + render_area.w && position.y > render_area.y && position.y < render_area.y + render_area.h
    }

    fn round_to_nearest(mut number: f32, nearest: f32) -> f32 {
        if (number % nearest) >= nearest / 2_f32 {
            number = number + (nearest - (number % nearest))
        } else {
            number = number - (number % nearest)
        }
        number
    }

    pub fn render(&mut self, player: &Player, textures: &BlockResources) {

        let render_area = Rect {
            x: player.position.x - (screen_width() / 2.) - 150_f32,
            y: player.position.y - (screen_height() / 2.) - 150_f32,
            w: screen_width() + 300_f32,
            h: screen_height() + 300_f32,
        };

        if !(self.prev_render_area == render_area) {
            let x_pos = Self::round_to_nearest(render_area.x, BLOCK_SIZE as f32) as i32;
            let y_pos = Self::round_to_nearest(render_area.y, BLOCK_SIZE as f32) as i32;
            let width = Self::round_to_nearest(render_area.w, BLOCK_SIZE as f32) as i32;
            let height = Self::round_to_nearest(render_area.h, BLOCK_SIZE as f32) as i32;


            for x_coord in (x_pos..x_pos + width).step_by(BLOCK_SIZE) {
                for y_coord in (y_pos..y_pos + height).step_by(BLOCK_SIZE) {
                    self.add_block_if_not_there(Block::new(
                        Self::i32_vec2(x_coord, y_coord),
                        self.terrain_generator.get_block(Self::i32_vec2(x_coord, y_coord)),
                    ));
                }
            }
        }

        self.blocks
            .retain(|block| Self::is_in_render_area(&render_area, block.position));

        self.prev_render_area = render_area;

        for block in self.blocks.iter() {
            draw_texture_ex(
                match block.block_type {
                    BlockType::Dirt => textures.dirt,
                    BlockType::Grass => textures.grass,
                    BlockType::Ice => textures.ice,
                    BlockType::Lava => textures.lava,
                    BlockType::Leaves => textures.leaves,
                    BlockType::Sand => textures.sand,
                    BlockType::Snow => textures.snow,
                    BlockType::Stone => textures.stone,
                    BlockType::Water => textures.water,
                    BlockType::WaterDeep => textures.water_deep,
                    BlockType::WoodLog => textures.wood_log,
                    BlockType::WoodPlanks => textures.wood_planks,
                },
                block.position.x,
                block.position.y,
                WHITE,
                DrawTextureParams {
                    dest_size: Option::from(vec2(BLOCK_SIZE as f32, BLOCK_SIZE as f32)),
                    rotation: (block.position.y % block.position.x) % FRAC_2_PI,
                    ..Default::default()
                },
            )
        }
    }

}
