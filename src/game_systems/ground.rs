use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::global_types::{
    AppState, DespawnWithLevel, GameBoundaries, GameOver, Ground, PlayerHead,
};
use crate::ui::MenuType;
use crate::utils::entities_ordered_by_type;

pub struct GroundPlugin;

impl Plugin for GroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::LoadLevel).with_system(add_ground));
        app.add_system_set(SystemSet::on_update(AppState::Game).with_system(detect_ground_touch));
    }
}

fn add_ground(mut commands: Commands, game_boundaries: Res<GameBoundaries>) {
    let mut cmd = commands.spawn_bundle(RigidBodyBundle {
        body_type: RigidBodyType::Static.into(),
        ..Default::default()
    });
    cmd.insert_bundle(ColliderBundle {
        shape: ColliderShape::cuboid(game_boundaries.width() * 0.5, 1.0).into(),
        position: Vec2::new(game_boundaries.center(), -0.5).into(),
        ..Default::default()
    });
    cmd.insert(ColliderDebugRender::with_id(1));
    cmd.insert(ColliderPositionSync::Discrete);
    cmd.insert(DespawnWithLevel);
    cmd.insert(Ground);

    let every = 1.0;
    let how_many = (game_boundaries.width() / every) as u32;
    for i in 1..how_many {
        commands.spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::YELLOW,
                custom_size: Some(Vec2::new(0.5, 0.1)),
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(game_boundaries.left + every * i as f32, -0.5, 0.5),
                ..Default::default()
            },
            ..Default::default()
        });
    }
}

fn detect_ground_touch(
    mut reader: EventReader<ContactEvent>,
    ground_query: Query<(), With<Ground>>,
    player_head_query: Query<(), With<PlayerHead>>,
    mut menu_writer: EventWriter<MenuType>,
) {
    for event in reader.iter() {
        if let ContactEvent::Started(handle1, handle2) = event {
            if entities_ordered_by_type!(
                [handle1.entity(), handle2.entity()],
                ground_query,
                player_head_query
            )
            .is_some()
            {
                menu_writer.send(MenuType::GameOver(GameOver::Injured));
            } else {
            }
        }
    }
}
