mod camera;
mod pogo;

use bevy::ecs::schedule::ShouldRun;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::components::DespawnWithLevel;
use crate::AppState;

pub struct GameSystemsPlugin;

fn create_move_to_state_system(new_state: AppState) -> impl Fn(ResMut<State<AppState>>) {
    move |mut state: ResMut<State<AppState>>| {
        state.set(new_state.clone()).unwrap();
    }
}

impl Plugin for GameSystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(camera::CameraPlugin);
        app.add_system_set({
            SystemSet::on_enter(AppState::ClearLevelAndThenLoad)
                .with_system(clear_level)
                .with_system(create_move_to_state_system(AppState::LoadLevel))
        });
        app.add_system_set({
            SystemSet::on_enter(AppState::LoadLevel)
                .with_system(add_ground)
                .with_system(create_move_to_state_system(AppState::Game))
        });
        app.add_plugin(pogo::PogoPlugin);
        app.add_system(enable_disable_physics.with_run_criteria(run_on_state_change));
    }
}

pub fn run_on_state_change(state: Res<State<AppState>>, mut prev_state: Local<Option<AppState>>) -> ShouldRun {
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

fn add_ground(mut commands: Commands) {
    let mut cmd = commands.spawn_bundle(RigidBodyBundle {
        body_type: RigidBodyType::Static.into(),
        ..Default::default()
    });
    cmd.insert_bundle(ColliderBundle {
        shape: ColliderShape::cuboid(100.0, 1.0).into(),
        position: Vec2::new(90.0, -0.5).into(),
        ..Default::default()
    });
    cmd.insert(ColliderDebugRender::with_id(1));
    cmd.insert(ColliderPositionSync::Discrete);
    cmd.insert(DespawnWithLevel);

    for i in -4..90 {
        commands.spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::YELLOW,
                custom_size: Some(Vec2::new(0.5, 0.1)),
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(2.0 * i as f32, -0.5, 0.5),
                ..Default::default()
            },
            ..Default::default()
        });
    }
}

fn enable_disable_physics(
    state: Res<State<AppState>>,
    mut rapier_configuration: ResMut<bevy_rapier2d::physics::RapierConfiguration>,
) {
    match state.current() {
        AppState::Menu | AppState::ClearLevelAndThenLoad | AppState::LoadLevel => {
            rapier_configuration.physics_pipeline_active = false;
            rapier_configuration.query_pipeline_active = false;
        }
        AppState::Game => {
            rapier_configuration.physics_pipeline_active = true;
            rapier_configuration.query_pipeline_active = true;
        }
    }
}
