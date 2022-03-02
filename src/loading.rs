use bevy::prelude::*;
use bevy_asset_loader::{AssetCollection, AssetCollectionApp};
// use bevy_kira_audio::AudioSource;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.init_collection::<FontAssets>();
        app.init_collection::<TextureAssets>();
    }
}

#[derive(AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub fira_sans: Handle<Font>,
}

#[derive(AssetCollection)]
pub struct AudioAssets {}

#[derive(AssetCollection)]
pub struct TextureAssets {
    #[asset(path = "sprites/pogo-player.png")]
    pub pogo_player: Handle<Image>,
    #[asset(path = "sprites/hurdle.png")]
    pub hurdle: Handle<Image>,
}
