use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::global_types::{AppState, DespawnWithLevel};
use crate::loading::AnimationAssets;

pub struct CompetitorsPlugin;

impl Plugin for CompetitorsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::LoadLevel).with_system(add_competitors));
        //app.add_system_set(SystemSet::on_update(AppState::Game).with_system(detect_hurdle_touch));
    }
}

fn add_competitors(
    mut commands: Commands,
    animation_assets: Res<AnimationAssets>,
) {
    let mut cmd = commands.spawn();
    cmd.insert_bundle(SpriteSheetBundle {
        sprite: TextureAtlasSprite {
            custom_size: Some(Vec2::new(2.0, 2.0)),
            ..Default::default()
        }.into(),
        texture_atlas: animation_assets.competitor_atlas.clone(),
        ..Default::default()
    });
    cmd.insert_bundle(RigidBodyBundle {
        body_type: RigidBodyType::Dynamic.into(),
        position: point![0.0, 1.0].into(),
        velocity: RigidBodyVelocity {
            linvel: Vec2::new(3.0, 0.0).into(),
            angvel: 0.0,
        }.into(),
        ..Default::default()
    });
    cmd.insert_bundle(ColliderBundle {
        shape: ColliderShape::cuboid(0.31, 1.0).into(),
        mass_properties: MassProperties::new(Default::default(), 10.0, 0.0).into(),
        material: ColliderMaterial {
            friction: 0.0,
            friction_combine_rule: CoefficientCombineRule::Min,
            ..Default::default()
        }.into(),
        ..Default::default()
    });
    cmd.insert(DespawnWithLevel);
    cmd.insert(RigidBodyPositionSync::Discrete);
    cmd.insert(animation_assets.competitor.clone());
    cmd.insert(benimator::Play);
}
