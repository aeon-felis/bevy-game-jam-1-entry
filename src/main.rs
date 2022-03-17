// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::{App, ClearColor, Color, Msaa, WindowDescriptor};
use bevy::DefaultPlugins;
use bevy_rapier2d::physics::{NoUserData, RapierPhysicsPlugin};
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
    app.add_plugins(DefaultPlugins);
    app.add_plugin(GamePlugin);
    app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default());
    app.add_plugin(benimator::AnimationPlugin::default());
    app.add_plugin(bevy_egui_kbgp::bevy_egui::EguiPlugin);
    app.insert_resource(bevy_egui_kbgp::bevy_egui::EguiSettings { scale_factor: 2.0 });
    app.add_system(bevy_egui_kbgp::kbgp_system_default_input);
    app.run();
}
