use bevy::prelude::*;
use bevy_asset_loader::{AssetCollection, AssetCollectionApp};
// use bevy_kira_audio::AudioSource;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.init_collection::<FontAssets>();
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
pub struct TextureAssets {}
