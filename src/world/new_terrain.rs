use bracket_noise::prelude::FastNoise;
use bracket_noise::prelude::NoiseType;
use macroquad::math::Vec2;

use crate::world::block::Block;
use crate::world::terrain::BlockType;
use crate::world::terrain::BlockType::*;
use crate::world::terrain::TerrainGenerator;

use crate::world::terrain::{NOISE_SCALING_FACTOR, SEED};

#[allow(dead_code)]
pub struct NewTerrain {
    pub mountain_noise_cellular: FastNoise,
    pub land_or_sea_perlin: FastNoise,
    pub tree_perlin: FastNoise,
}

impl NewTerrain {
    pub fn new() -> NewTerrain {
        let mut mountain_noise_cellular = FastNoise::seeded(SEED);
        mountain_noise_cellular.set_noise_type(NoiseType::Cubic);
        mountain_noise_cellular.set_frequency(0.001);

        let mut land_or_sea_perlin = FastNoise::seeded(SEED);
        land_or_sea_perlin.set_noise_type(NoiseType::Perlin);
        land_or_sea_perlin.set_frequency(0.5);

        let mut tree_perlin = FastNoise::seeded(SEED);
        tree_perlin.set_noise_type(NoiseType::Perlin);
        tree_perlin.set_frequency(5.);
        NewTerrain {
            mountain_noise_cellular,
            land_or_sea_perlin,
            tree_perlin,
        }
    }
}

impl TerrainGenerator for NewTerrain {
    fn get_block_type(&self, position: Vec2) -> BlockType {
        let land_or_sea = self.land_or_sea_perlin.get_noise(
            position.x / NOISE_SCALING_FACTOR,
            position.y / NOISE_SCALING_FACTOR,
        );
        if land_or_sea < -0.1 {
            //its water
            Water
        } else if land_or_sea < -0.07 {
            Sand
        } else {
            //its land
            //generate some mountains :sungalasses:
            let mountain_noise = self
                .mountain_noise_cellular
                .get_noise(position.x, position.y);
            if mountain_noise < -0.5 {
                //snowy
                Snow
            } else if mountain_noise < 0.2 {
                //its a mountain
                Stone
            } else {
                //its not a mountain
                let tree_noise = self.tree_perlin.get_noise(
                    position.x / NOISE_SCALING_FACTOR,
                    position.y / NOISE_SCALING_FACTOR,
                );
                if tree_noise < 0. {
                    //tree
                    Leaves
                } else {
                    //not a tree
                    Grass
                }
            }
        }
    }

    fn get_block_shadow(&self, position: Vec2) -> u8 {
        let at_pos = self
            .mountain_noise_cellular
            .get_noise(position.x, position.y);
        let below = self
            .mountain_noise_cellular
            .get_noise(position.x, position.y - 1.);
        if at_pos > below { 1 } else { 0 }
    }
    fn get_block(&self, position: Vec2) -> Block {
        Block::new(
            position,
            self.get_block_type(position),
            self.get_block_shadow(position),
        )
    }
}
