use std::collections::HashMap;

use bracket_noise::prelude::FastNoise;
use bracket_noise::prelude::NoiseType;
use macroquad::math::Vec2;
use macroquad::math::vec2;

use crate::scenes::planet_surface::renderer::BLOCK_SIZE;
use crate::utils::round_to_nearest;
use crate::utils::vec2_i32_pair;
use crate::world::block::Block;
use crate::world::terrain::BlockType::*;
use crate::world::terrain::MAX_HEIGHT;
use crate::world::terrain::TerrainGenerator;

use crate::world::terrain::{ SEED};

#[allow(dead_code)]
pub struct NewTerrain {
    pub height_noise: FastNoise,
    pub tree_perlin: FastNoise,
    height_map: HashMap<(i32, i32), i32>,
}

impl NewTerrain {
    pub fn new() -> NewTerrain {
        let mut height_noise = FastNoise::seeded(SEED);
        height_noise.set_noise_type(NoiseType::Perlin);
        height_noise.set_frequency(0.001);

        let mut tree_perlin = FastNoise::seeded(SEED);
        tree_perlin.set_noise_type(NoiseType::Perlin);
        tree_perlin.set_frequency(5.);
        NewTerrain {
            height_noise,
            tree_perlin,
            height_map: HashMap::new(),
        }
    }
}

impl NewTerrain {
    fn height_at(&mut self, position: Vec2) -> i32 {
        let pair = vec2_i32_pair(position);
        if self.height_map.contains_key(&pair) {
            return self.height_map[&pair];
        }

        let mut val = ((self.height_noise.get_noise(position.x, position.y) + 1.) * MAX_HEIGHT / 2.) as i32;
        if val < 0{
            val = 0;
        }
        if val > MAX_HEIGHT as i32{
            val = MAX_HEIGHT as i32;
        }
        self.height_map.insert(pair, val);
        val
    }


    fn get_shadow_lvl(&mut self, position: Vec2) -> u8{
        let here = self.height_at(position);
        let above = self.height_at(vec2(position.x, position.y - BLOCK_SIZE as f32));
        if above > here{
            1
        }else{
            0
        }
    }
}

impl TerrainGenerator for NewTerrain {
    fn get_block(&mut self, position: Vec2) -> Block {
        println!("Pos: {}", position);
        let height = self.height_at(position);
        let shadow = self.get_shadow_lvl(position);
        if height < 30{
            Block::new_height(position, Water, shadow, height)
        }
        else if height < 32{
            Block::new_height(position, Sand, shadow, height)
        }else if height < 60{
            Block::new_height(position, Grass, shadow, height)
        }else if height < 100{
            Block::new_height(position, Stone, shadow, height)
        }else {
            Block::new_height(position, Snow, shadow, height)
        }
    }
}
