mod audio;
mod components;
mod game_systems;
mod loading;
mod ui;

use crate::audio::InternalAudioPlugin;
use crate::loading::LoadingPlugin;

use bevy::app::App;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;

#[derive(Hash, Debug, PartialEq, Eq, Clone)]
pub enum AppState {
    Menu,
    ClearLevelAndThenLoad,
    LoadLevel,
    Game,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(AppState::Menu);
        app.add_plugin(LoadingPlugin);
        app.add_plugin(InternalAudioPlugin);
        app.add_plugin(ui::UiPlugin);
        app.add_plugin(game_systems::GameSystemsPlugin);

        #[cfg(debug_assertions)]
        {
            app.add_plugin(FrameTimeDiagnosticsPlugin::default());
            app.add_plugin(LogDiagnosticsPlugin::default());
        }
    }
}
