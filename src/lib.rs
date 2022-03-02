mod audio;
mod consts;
mod game_systems;
mod global_types;
mod loading;
mod ui;
mod utils;

use crate::audio::InternalAudioPlugin;
use crate::loading::LoadingPlugin;

use bevy::app::App;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;

use self::consts::TRACK_LENGTH;
use self::global_types::{AppState, GameBoundaries, GameOver, PlayerStatus};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(AppState::Menu);
        app.add_state::<Option<GameOver>>(None);
        app.insert_resource(GameBoundaries {
            left: -10.0,
            right: TRACK_LENGTH,
        });
        app.init_resource::<PlayerStatus>();
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
