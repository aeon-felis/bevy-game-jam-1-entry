mod audio;
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
    Game,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(AppState::Menu);
        app.add_plugin(LoadingPlugin);
        app.add_plugin(InternalAudioPlugin);
        app.add_plugin(ui::UiPlugin);

        #[cfg(debug_assertions)]
        {
            app.add_plugin(FrameTimeDiagnosticsPlugin::default());
            app.add_plugin(LogDiagnosticsPlugin::default());
        }
    }
}
