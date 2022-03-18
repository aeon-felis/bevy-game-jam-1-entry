use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::global_types::{
    AppState, DespawnWithLevel, GameBoundaries, GameOver, Ground, MenuState, PlayerHead,
};
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
        position: Vec2::new(game_boundaries.center(), -1.0).into(),
        ..Default::default()
    });
    cmd.insert(ColliderPositionSync::Discrete);
    cmd.insert(DespawnWithLevel);
    cmd.insert(Ground);

    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb_u8(185, 113, 124),
            custom_size: Some(Vec2::new(game_boundaries.width(), 2.0)),
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(game_boundaries.center(), 0.0, -0.1),
            ..Default::default()
        },
        ..Default::default()
    });

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
                translation: Vec3::new(game_boundaries.left + every * i as f32, -0.05, 0.5),
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
    mut state: ResMut<State<AppState>>,
    mut game_over_state: ResMut<State<Option<GameOver>>>,
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
                state.set(AppState::Menu(MenuState::GameOver)).unwrap();
                game_over_state.set(Some(GameOver::Injured)).unwrap();
            } else {
            }
        }
    }
}
