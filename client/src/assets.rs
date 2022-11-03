use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection)]
pub struct GameAssets {
    #[asset(path = "game/arenascreen.png")]
    pub arena_background: Handle<Image>,
    #[asset(path = "game/characters/archer/arrows", collection(typed))]
    pub archer_arrows: Vec<Handle<Image>>,
    #[asset(texture_atlas(tile_size_x = 10., tile_size_y = 10., columns = 17, rows = 1))]
    #[asset(path = "game/characters/archer/archer_spritesheet.png")]
    pub archer_tileset: Handle<TextureAtlas>,
    #[asset(path = "game/characters/archer/arrow_noise.ogg")]
    pub arrow_noise: Handle<AudioSource>,
}

#[derive(AssetCollection)]
pub struct MainMenuAssets {
    #[asset(path = "mainmenu/alagrad.ttf")]
    pub alagrad: Handle<Font>,
    #[asset(path = "mainmenu/mainmenuscreen.png")]
    pub main_menu_screen: Handle<Image>,
    #[asset(path = "mainmenu/ui_button.ogg")]
    pub ui_button: Handle<AudioSource>,
}