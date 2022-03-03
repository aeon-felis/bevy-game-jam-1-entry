use std::time::Duration;

use benimator::SpriteSheetAnimation;
use bevy::prelude::*;
use bevy_asset_loader::{AssetCollection, AssetCollectionApp};
// use bevy_kira_audio::AudioSource;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.init_collection::<FontAssets>();
        app.init_collection::<TextureAssets>();
        app.add_startup_system(init_animations);
        app.init_resource::<AnimationAssets>();
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
    #[asset(path = "sprites/competitor.png")]
    pub competitor: Handle<Image>,
}

#[derive(Default)]
pub struct AnimationAssets {
    pub competitor_atlas: Handle<TextureAtlas>,
    pub competitor: Handle<SpriteSheetAnimation>,
}

fn init_animations(
    mut animation_assets: ResMut<AnimationAssets>,
    texture_assets: Res<TextureAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut animations: ResMut<Assets<SpriteSheetAnimation>>,
) {
    animation_assets.competitor_atlas = texture_atlases.add(TextureAtlas::from_grid(
        texture_assets.competitor.clone(),
        Vec2::new(64.0, 64.0),
        4,
        1,
    ));
    animation_assets.competitor = animations.add(SpriteSheetAnimation::from_range(
        0..=3,
        Duration::from_millis(200),
    ));
}
