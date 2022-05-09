use macroquad::prelude::*;
use crate::miniquad::log;

pub struct BlockResources {
    pub dirt: Texture2D,
    pub grass: Texture2D,
    pub ice: Texture2D,
    pub lava: Texture2D,
    pub leaves: Texture2D,
    pub sand: Texture2D,
    pub snow: Texture2D,
    pub stone: Texture2D,
    pub water: Texture2D,
    pub water_deep: Texture2D,
    pub wood_log: Texture2D,
    pub wood_planks: Texture2D,
}

macro_rules! load_blocks {
    ($($var_name:ident),+) => {
        let default = macroquad::prelude::load_texture("assets/blocks/error.png").await.unwrap();
        $(
            let var_name_str = stringify!($var_name);
            let path = format!("assets/blocks/{}.png", var_name_str);
            let $var_name = macroquad::prelude::load_texture(path.as_str()).await.unwrap_or(default);
            $var_name.set_filter(macroquad::prelude::FilterMode::Nearest);
        )+
    };
}

macro_rules! load {
    ($($var_name:ident),+) => {
        $(
            let var_name_str = stringify!($var_name);
            let path = format!("assets/{}.png", var_name_str);
            let $var_name = macroquad::prelude::load_texture(path.as_str()).await.unwrap();
            $var_name.set_filter(macroquad::prelude::FilterMode::Nearest);
        )+
    };
}

impl BlockResources {
    pub async fn load() -> Result<BlockResources, FileError> {
        info!("Loading resources...");

        load_blocks!(dirt, grass, ice, lava, leaves, sand, snow, stone, water, water_deep, wood_log, wood_planks);

        info!("Done loading resources");

        Ok(BlockResources {
            dirt,
            grass,
            ice,
            lava,
            leaves,
            sand,
            snow,
            stone,
            water,
            water_deep,
            wood_log,
            wood_planks,
        })
    }
}

pub struct Resources {
    pub player_texture: Texture2D,
    pub font: Font,
    pub block_resources: BlockResources,
}

impl Resources {
    pub async fn load() -> Result<Resources, FileError> {
        load!(player_texture);

        let font = load_ttf_font("assets/Font.ttf").await.unwrap();

        let block_resources = BlockResources::load().await.unwrap();
        Ok(Resources {
            player_texture,
            font,
            block_resources,
        })
    }
}
