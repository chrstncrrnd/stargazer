use macroquad::prelude::*;

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

impl BlockResources {
    pub async fn new() -> Result<BlockResources, FileError> {
        let dirt = load_texture("assets/blocks/dirt.png").await?;
        dirt.set_filter(FilterMode::Nearest);

        let grass = load_texture("assets/blocks/grass.png").await?;
        grass.set_filter(FilterMode::Nearest);

        let ice = load_texture("assets/blocks/sand.png").await?;
        ice.set_filter(FilterMode::Nearest);

        let lava = load_texture("assets/blocks/lava.png").await?;
        lava.set_filter(FilterMode::Nearest);

        let leaves = load_texture("assets/blocks/leaves.png").await?;
        leaves.set_filter(FilterMode::Nearest);

        let sand = load_texture("assets/blocks/sand.png").await?;
        sand.set_filter(FilterMode::Nearest);

        let snow = load_texture("assets/blocks/snow.png").await?;
        snow.set_filter(FilterMode::Nearest);

        let stone = load_texture("assets/blocks/stone.png").await?;
        stone.set_filter(FilterMode::Nearest);

        let water = load_texture("assets/blocks/water.png").await?;
        water.set_filter(FilterMode::Nearest);

        let water_deep = load_texture("assets/blocks/water_deep.png").await?;
        water_deep.set_filter(FilterMode::Nearest);

        let wood_log = load_texture("assets/blocks/wood_log.png").await?;
        wood_log.set_filter(FilterMode::Nearest);

        let wood_planks = load_texture("assets/blocks/wood_planks.png").await?;
        wood_planks.set_filter(FilterMode::Nearest);

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
    pub async fn new() -> Result<Resources, FileError> {
        let player_texture = load_texture("assets/player_texture_default.png").await?;
        player_texture.set_filter(FilterMode::Nearest);

        let font = load_ttf_font("assets/Font.ttf").await.unwrap();

        let block_resources = BlockResources::new().await.unwrap();
        Ok(Resources {
            player_texture,
            font,
            block_resources,
        })
    }
}
