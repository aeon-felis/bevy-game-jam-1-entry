mod camera;
mod hurdles;
mod pogo;

use bevy::ecs::schedule::ShouldRun;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::components::DespawnWithLevel;
use crate::consts::TRACK_LENGTH;
use crate::{AppState, GameOver};

pub struct GameSystemsPlugin;

fn create_move_to_state_system(new_state: AppState) -> impl Fn(ResMut<State<AppState>>) {
    move |mut state: ResMut<State<AppState>>| {
        state.set(new_state.clone()).unwrap();
    }
}

impl Plugin for GameSystemsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameBoundaries {
            left: -10.0,
            right: TRACK_LENGTH,
        });
        app.add_plugin(camera::CameraPlugin);
        app.add_system_set({
            SystemSet::on_enter(AppState::ClearLevelAndThenLoad)
                .with_system(clear_level)
                .with_system(create_move_to_state_system(AppState::LoadLevel))
                .with_system(|mut game_over_state: ResMut<State<Option<GameOver>>>| {
                    let _ = game_over_state.set(None);
                })
        });
        app.add_system_set({
            SystemSet::on_enter(AppState::LoadLevel)
                .with_system(add_ground)
                .with_system(create_move_to_state_system(AppState::Game))
        });
        app.add_plugin(pogo::PogoPlugin);
        app.add_plugin(hurdles::HurdlesPlugin);
        app.add_system(enable_disable_physics.with_run_criteria(run_on_state_change));
    }
}

pub struct GameBoundaries {
    pub left: f32,
    pub right: f32,
}

impl GameBoundaries {
    pub fn width(&self) -> f32 {
        self.right - self.left
    }

    pub fn center(&self) -> f32 {
        (self.right + self.left) * 0.5
    }
}

pub fn run_on_state_change(
    state: Res<State<AppState>>,
    mut prev_state: Local<Option<AppState>>,
) -> ShouldRun {
    let state = state.current();
    if Some(state) == (&*prev_state).as_ref() {
        return ShouldRun::No;
    }
    *prev_state = Some(state.clone());
    return ShouldRun::Yes;
}

fn clear_level(mut commands: Commands, query: Query<Entity, With<DespawnWithLevel>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
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

fn enable_disable_physics(
    state: Res<State<AppState>>,
    mut rapier_configuration: ResMut<bevy_rapier2d::physics::RapierConfiguration>,
    game_over_state: Res<State<Option<GameOver>>>,
) {
    let set_to = match state.current() {
        AppState::Menu | AppState::ClearLevelAndThenLoad | AppState::LoadLevel => {
            game_over_state.current().is_some()
        }
        AppState::Game => {
            true
        }
    };
    rapier_configuration.physics_pipeline_active = set_to;
    rapier_configuration.query_pipeline_active = set_to;
}
