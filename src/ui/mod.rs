mod score;

use bevy::prelude::*;
use bevy_egui_kbgp::prelude::*;
use bevy_egui_kbgp::egui;
use bevy_egui_kbgp::bevy_egui::EguiContext;


use crate::global_types::MenuState;
use crate::global_types::{AppState, GameOver, PlayerStatus};
// use crate::loading::FontAssets;
use crate::ui::score::ScorePlugin;

pub struct UiPlugin;

#[derive(Component, Clone)]
enum MenuAction {
    StartGame,
    ResumeGame,
    BackToMainMenu,
    #[cfg_attr(target_arch = "wasm32", allow(unused))]
    ExitGame,
}

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_ui);
        app.add_event::<MenuAction>();

        app.add_plugin(ScorePlugin);

        app.add_system_set(SystemSet::on_update(AppState::Menu(MenuState::Main)).with_system(main_menu));
        app.add_system_set(SystemSet::on_update(AppState::Menu(MenuState::Pause)).with_system(pause_menu));
        app.add_system_set(SystemSet::on_update(AppState::Menu(MenuState::GameOver)).with_system(game_over_menu));
        app.add_system(handle_menu_actions);
        app.add_system(pause_unpause_game);
    }
}

fn setup_ui(mut commands: Commands) {
    commands.spawn_bundle(UiCameraBundle::default());
}

fn handle_menu_actions(
    mut reader: EventReader<MenuAction>,
    mut state: ResMut<State<AppState>>,
    mut exit: EventWriter<bevy::app::AppExit>,
) {
    for event in reader.iter() {
        match event {
            MenuAction::StartGame => {
                state.set(AppState::ClearLevelAndThenLoad).unwrap();
            }
            MenuAction::ExitGame => {
                exit.send(bevy::app::AppExit);
            }
            MenuAction::ResumeGame => {
                state.set(AppState::Game).unwrap();
            }
            MenuAction::BackToMainMenu => {
                state.set(AppState::Menu(MenuState::Main)).unwrap();
            }
        }
    }
}

fn pause_unpause_game(
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut state: ResMut<State<AppState>>,
    mut action_writer: EventWriter<MenuAction>,
) {
    if keyboard_input.pressed(KeyCode::Escape) {
        keyboard_input.reset(KeyCode::Escape);
    } else {
        return;
    }
    match state.current() {
        AppState::Menu(MenuState::Pause) => {
            action_writer.send(MenuAction::ResumeGame);
        }
        AppState::Menu(_) => {}
        AppState::ClearLevelAndThenLoad => {}
        AppState::LoadLevel => {}
        AppState::Game => {
            state.set(AppState::Menu(MenuState::Pause)).unwrap();
        }
    }
}

fn menu_layout(egui_context: &egui::Context, dlg: impl FnOnce(&mut egui::Ui)) {
    egui::CentralPanel::default().frame(egui::Frame::none()).show(egui_context, |ui| {
        let layout = egui::Layout::top_down(egui::Align::Center);
        ui.with_layout(layout, |ui| {
            dlg(ui);
        });
    });
}

fn main_menu(
    mut egui_context: ResMut<EguiContext>,
    mut action_writer: EventWriter<MenuAction>,
) {
    menu_layout(egui_context.ctx_mut(), |ui| {
        if ui.button("Start").kbgp_navigation().clicked() {
            action_writer.send(MenuAction::StartGame);
        }
        #[cfg(not(target_arch = "wasm32"))]
        if ui.button("Exit").kbgp_navigation().kbgp_activated() {
            action_writer.send(MenuAction::ExitGame);
        }
    });
}

fn pause_menu(
    mut egui_context: ResMut<EguiContext>,
    mut action_writer: EventWriter<MenuAction>,
) {
    menu_layout(egui_context.ctx_mut(), |ui| {
        if ui.button("Resume").kbgp_navigation().kbgp_activated() {
            action_writer.send(MenuAction::ResumeGame);
        }
        if ui.button("Main Menu").kbgp_navigation().kbgp_activated() {
            action_writer.send(MenuAction::BackToMainMenu);
            let focus = ui.memory().focus();
            if let Some(focus) = focus {
                ui.memory().surrender_focus(focus);
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        if ui.button("Exit").kbgp_navigation().kbgp_activated() {
            action_writer.send(MenuAction::ExitGame);
        }
    });
}

fn game_over_menu(
    mut egui_context: ResMut<EguiContext>,
    mut action_writer: EventWriter<MenuAction>,
    game_over_state: Res<State<Option<GameOver>>>,
    player_status: Res<PlayerStatus>,
) {
    menu_layout(egui_context.ctx_mut(), |ui| {
        if ui.button("Main Menu").kbgp_navigation().kbgp_activated() {
            action_writer.send(MenuAction::BackToMainMenu);
            let focus = ui.memory().focus();
            if let Some(focus) = focus {
                ui.memory().surrender_focus(focus);
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        if ui.button("Exit").kbgp_navigation().kbgp_activated() {
            action_writer.send(MenuAction::ExitGame);
        }
        if let Some(game_over) = game_over_state.current() {
            match game_over {
                GameOver::Injured => {
                    ui.label("INJURED!"); // TODO: Make this red
                    ui.label(format!("Traveled {:.1}m before hitting your head", player_status.distance_traveled));
                }
                GameOver::Disqualified => {
                    ui.label("DISQUALIFIED!"); // TODO: Make this red
                    ui.label(format!("Traveled {:.1}m before hitting a hurdle", player_status.distance_traveled));
                }
                GameOver::WrongWay => {
                    ui.label("that's the wrong way..."); // TODO: Make this red
                }
                GameOver::FinishLine => {
                    ui.label("FINISH!"); // TODO: Make this green
                    ui.label(format!(
                            "Finished in {}, place {} out of {}",
                            player_status.format_time(),
                            player_status.place(),
                            player_status.total_runners(),
                    ));
                }
            }
        }
    });
}
