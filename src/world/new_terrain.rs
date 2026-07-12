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
    fn get_block(&self, position: Vec2) -> Block {
        let land_or_sea = self.land_or_sea_perlin.get_noise(
            position.x / NOISE_SCALING_FACTOR,
            position.y / NOISE_SCALING_FACTOR,
        );
        if land_or_sea < -0.1 {
            //its water
            Block::new(position, Water, 0)
        } else if land_or_sea < -0.07 {
            Block::new(position, Sand, 0)
        } else {
            //its land
            //generate some mountains :sungalasses:
            let mountain_noise = self
                .mountain_noise_cellular
                .get_noise(position.x, position.y);
            if mountain_noise < -0.5 {
                //snowy
                Block::new(position, Snow, 0)
            } else if mountain_noise < 0.2 {
                //its a mountain
                Block::new(position, Stone, 0)
            } else {
                //its not a mountain
                let tree_noise = self.tree_perlin.get_noise(
                    position.x / NOISE_SCALING_FACTOR,
                    position.y / NOISE_SCALING_FACTOR,
                );
                if tree_noise < 0. {
                    //tree
                    Block::new(position, Leaves, 0)
                } else {
                    //not a tree
                    Block::new(position, Grass, 0)
                }
            }
        }
    }

}
