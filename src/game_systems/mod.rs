mod pogo;

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
        app.add_startup_system(setup_camera);
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
    }
}

fn setup_camera(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.transform.scale.x *= 0.02;
    camera.transform.scale.y *= 0.02;
    camera.transform.translation.y = 2.0;
    commands.spawn_bundle(camera);
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
        position: Vec2::new(0.0, -0.5).into(),
        ..Default::default()
    });
    cmd.insert(ColliderDebugRender::with_id(1));
    cmd.insert(ColliderPositionSync::Discrete);
    cmd.insert(DespawnWithLevel);
}
