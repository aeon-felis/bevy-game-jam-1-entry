mod camera;
mod competitors;
mod ground;
mod hurdles;
mod pogo;

use bevy::ecs::schedule::ShouldRun;
use bevy::prelude::*;

use crate::global_types::{AppState, DespawnWithLevel, GameOver, MenuState, PlayerStatus};

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
                .with_system(reset_resources)
                .with_system(create_move_to_state_system(AppState::LoadLevel))
        });
        app.add_system_set(
            SystemSet::on_enter(AppState::LoadLevel)
                .with_system(create_move_to_state_system(AppState::Game)),
        );
        app.add_plugin(ground::GroundPlugin);
        app.add_plugin(pogo::PogoPlugin);
        app.add_plugin(hurdles::HurdlesPlugin);
        app.add_plugin(competitors::CompetitorsPlugin);
        app.add_system(enable_disable_physics.with_run_criteria(run_on_state_change));
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

fn reset_resources(
    mut game_over_state: ResMut<State<Option<GameOver>>>,
    mut player_status: ResMut<PlayerStatus>,
) {
    let _ = game_over_state.set(None);
    *player_status = PlayerStatus::default();
}

fn enable_disable_physics(
    state: Res<State<AppState>>,
    mut rapier_configuration: ResMut<bevy_rapier2d::physics::RapierConfiguration>,
) {
    let set_to = match state.current() {
        AppState::Game => true,
        AppState::Menu(MenuState::GameOver) => true,
        AppState::Menu(_) | AppState::ClearLevelAndThenLoad | AppState::LoadLevel => false,
    };
    rapier_configuration.physics_pipeline_active = set_to;
    rapier_configuration.query_pipeline_active = set_to;
}
