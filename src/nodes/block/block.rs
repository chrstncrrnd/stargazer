use macroquad::prelude::*;

use crate::resources::BlockResources;

pub enum Blocks {
    Dirt,
    Grass,
    Sand,
    Error,
}

pub struct Block {
    pub position: Vec2,
    pub size: f32,
    pub block_type: Blocks,
}

impl Default for Block {
    fn default() -> Self {
        Block {
            position: Vec2::new(0.0, 0.0),
            size: 70.,
            block_type: Blocks::Error,
        }
    }
}

impl Block {
    pub fn new(position_: Vec2, block_type_: Blocks) -> Self {
        Block {
            position: position_,
            size: 70.,
            block_type: block_type_,
        }
    }

    pub fn render(&self, textures: &BlockResources) {
        draw_texture_ex(
            match self.block_type {
                Blocks::Dirt => textures.dirt,
                Blocks::Grass => textures.grass,
                Blocks::Sand => textures.sand,
                Blocks::Error => textures.error,
            },
            self.position.x,
            self.position.y,
            WHITE,
            DrawTextureParams {
                dest_size: Option::from(vec2(self.size, self.size)),
                ..Default::default()
            },
        );
    }
}
