// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::{App, ClearColor, Color, Msaa, WindowDescriptor};
use bevy::DefaultPlugins;
use bevy_rapier2d::physics::{NoUserData, RapierPhysicsPlugin};
use bevy_ui_navigation::NavigationPlugin;
use pogo_hurdling::GamePlugin;

fn main() {
    let mut app = App::new();
    app.insert_resource(Msaa { samples: 1 });
    app.insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)));
    app.insert_resource(WindowDescriptor {
        width: 800.,
        height: 600.,
        title: "Pogo Hurdling".to_string(),
        ..Default::default()
    });
    app.add_plugins_with(DefaultPlugins, |group| {
        #[cfg(not(debug_assertions))]
        group.add_before::<bevy::asset::AssetPlugin, _>(bevy_embedded_assets::EmbeddedAssetPlugin);
        group
    });
    app.add_plugin(GamePlugin);
    app.add_plugin(NavigationPlugin);
    app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default());
    app.run();
}
