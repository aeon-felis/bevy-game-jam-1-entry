mod score;
mod input;

use bevy::prelude::*;
use bevy_egui_kbgp::bevy_egui::EguiContext;
use bevy_egui_kbgp::egui;
use bevy_egui_kbgp::prelude::*;
use ezinput::prelude::*;

use crate::global_types::InputBinding;
use crate::global_types::MenuState;
use crate::global_types::{AppState, GameOver, PlayerStatus};
// use crate::loading::FontAssets;
use crate::ui::score::ScorePlugin;
use crate::ui::input::InputPlugin;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_ui);

        app.add_plugin(ScorePlugin);
        app.add_plugin(InputPlugin);

        app.add_system_set(
            SystemSet::on_update(AppState::Menu(MenuState::Main)).with_system(main_menu),
        );
        app.add_system_set(
            SystemSet::on_update(AppState::Menu(MenuState::Pause)).with_system(pause_menu),
        );
        app.add_system_set(
            SystemSet::on_update(AppState::Menu(MenuState::GameOver)).with_system(game_over_menu),
        );
        app.add_system_set(
            SystemSet::on_exit(AppState::Menu(MenuState::GameOver)).with_system(
                |mut game_over_state: ResMut<State<Option<GameOver>>>| {
                    game_over_state.set(None).unwrap();
                },
            ),
        );
        app.add_system(pause_unpause_game);
    }
}

fn setup_ui(mut commands: Commands) {
    commands.spawn_bundle(UiCameraBundle::default());
}

fn pause_unpause_game(
    input_views: Query<&InputView<InputBinding>>,
    mut state: ResMut<State<AppState>>,
) {
    if input_views.iter().any(|input_view| input_view.key(&InputBinding::Pause).just_pressed()) {
        match state.current() {
            AppState::Menu(MenuState::Pause) => {
                state.set(AppState::Game).unwrap();
            }
            AppState::Menu(_) => {}
            AppState::ClearLevelAndThenLoad => {}
            AppState::LoadLevel => {}
            AppState::Game => {
                state.set(AppState::Menu(MenuState::Pause)).unwrap();
            }
        }
    }
}

fn menu_layout(egui_context: &egui::Context, dlg: impl FnOnce(&mut egui::Ui)) {
    egui::CentralPanel::default()
        .frame(egui::Frame::none())
        .show(egui_context, |ui| {
            let layout = egui::Layout::top_down(egui::Align::Center);
            ui.with_layout(layout, |ui| {
                dlg(ui);
            });
        });
}

fn main_menu(
    mut egui_context: ResMut<EguiContext>,
    mut state: ResMut<State<AppState>>,
    #[cfg(not(target_arch = "wasm32"))] mut exit: EventWriter<bevy::app::AppExit>,
) {
    menu_layout(egui_context.ctx_mut(), |ui| {
        if ui
            .button("Start")
            .kbgp_navigation()
            .kbgp_initial_focus()
            .clicked()
        {
            state.set(AppState::ClearLevelAndThenLoad).unwrap();
        }
        #[cfg(not(target_arch = "wasm32"))]
        if ui.button("Exit").kbgp_navigation().clicked() {
            exit.send(bevy::app::AppExit);
        }
    });
}

fn pause_menu(
    mut egui_context: ResMut<EguiContext>,
    mut state: ResMut<State<AppState>>,
    #[cfg(not(target_arch = "wasm32"))] mut exit: EventWriter<bevy::app::AppExit>,
) {
    menu_layout(egui_context.ctx_mut(), |ui| {
        if ui
            .button("Resume")
            .kbgp_navigation()
            .kbgp_initial_focus()
            .clicked()
        {
            state.set(AppState::Game).unwrap();
        }
        if ui.button("Main Menu").kbgp_navigation().clicked() {
            state.set(AppState::Menu(MenuState::Main)).unwrap();
            ui.kbgp_clear_input();
        }
        #[cfg(not(target_arch = "wasm32"))]
        if ui.button("Exit").kbgp_navigation().clicked() {
            exit.send(bevy::app::AppExit);
        }
    });
}

fn game_over_menu(
    mut egui_context: ResMut<EguiContext>,
    mut state: ResMut<State<AppState>>,
    #[cfg(not(target_arch = "wasm32"))] mut exit: EventWriter<bevy::app::AppExit>,
    game_over_state: Res<State<Option<GameOver>>>,
    player_status: Res<PlayerStatus>,
) {
    menu_layout(egui_context.ctx_mut(), |ui| {
        if ui
            .button("Main Menu")
            .kbgp_navigation()
            .kbgp_initial_focus()
            .clicked()
        {
            state.set(AppState::Menu(MenuState::Main)).unwrap();
            ui.kbgp_clear_input();
        }
        #[cfg(not(target_arch = "wasm32"))]
        if ui.button("Exit").kbgp_navigation().clicked() {
            exit.send(bevy::app::AppExit);
        }
        if let Some(game_over) = game_over_state.current() {
            ui.style_mut().visuals.override_text_color = Some(egui::Color32::WHITE);
            match game_over {
                GameOver::Injured => {
                    ui.colored_label(egui::Color32::RED, "INJURED!");
                    ui.label(format!(
                        "Traveled {:.1}m before hitting your head",
                        player_status.distance_traveled
                    ));
                }
                GameOver::Disqualified => {
                    ui.colored_label(egui::Color32::RED, "DISQUALIFIED!");
                    ui.label(format!(
                        "Traveled {:.1}m before hitting a hurdle",
                        player_status.distance_traveled
                    ));
                }
                GameOver::WrongWay => {
                    ui.colored_label(egui::Color32::RED, "that's the wrong way...");
                }
                GameOver::FinishLine => {
                    ui.colored_label(egui::Color32::GREEN, "FINISH!");
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
