use bevy::prelude::*;
use bevy_ui_navigation::systems::InputMapping;
use bevy_ui_navigation::{FocusState, Focusable, NavEvent, NavRequest};
use ezinput::prelude::{ActionBinding, BindingInputReceiver, BindingTypeView, InputView};

use crate::global_types::{AppState, GameOver};
use crate::loading::FontAssets;

pub struct UiPlugin;

#[derive(Component, Clone)]
pub enum MenuType {
    Main,
    Pause,
    GameOver(GameOver),
}

#[derive(Component, Clone)]
enum MenuAction {
    StartGame,
    ResumeGame,
    BackToMainMenu,
    ExitGame,
}

#[derive(ezinput_macros::BindingTypeView, PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum UiBinding {
    Pause,
}

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        #[derive(SystemLabel, PartialEq, Eq, Debug, Hash, Clone)]
        struct SpawnMenuLabel;

        app.add_startup_system(setup_ui);
        app.init_resource::<InputMapping>();
        app.add_event::<MenuType>();
        app.add_event::<MenuAction>();
        app.add_system(spawn_menu.label(SpawnMenuLabel));
        app.add_startup_system(|mut writer: EventWriter<MenuType>| {
            writer.send(MenuType::Main);
        });
        app.add_system_set({
            SystemSet::on_update(AppState::Menu)
                .with_system(bevy_ui_navigation::systems::default_keyboard_input)
                .with_system(bevy_ui_navigation::systems::default_mouse_input)
                .with_system(bevy_ui_navigation::systems::default_gamepad_input)
        });
        app.add_system(focus_default.before(SpawnMenuLabel));
        app.add_system(handle_nav_events);
        app.add_system(handle_menu_actions);
        app.add_system_set(
            SystemSet::on_exit(AppState::Menu).with_system(destroy_menu_on_stage_exit),
        );
        app.add_system(pause_unpause_game);
        app.add_plugin(ezinput::prelude::EZInputPlugin::<UiBinding>::default());
    }
}

fn setup_ui(mut commands: Commands) {
    commands.spawn_bundle(UiCameraBundle::default());
    let mut view = InputView::empty();
    view.add_binding(UiBinding::Pause, &{
        let mut binding = ActionBinding::from(UiBinding::Pause);
        binding.receiver(BindingInputReceiver::KeyboardKey(KeyCode::Escape));
        binding.receiver(BindingInputReceiver::GamepadButton(
            GamepadButtonType::Start,
        ));
        binding
    });
    commands
        .spawn()
        .insert(view)
        .insert(ezinput::prelude::EZInputKeyboardService::default())
        .insert(ezinput::prelude::EZInputGamepadService::default());
}

fn spawn_menu(
    mut reader: EventReader<MenuType>,
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    mut state: ResMut<State<AppState>>,
    mut game_over_state: ResMut<State<Option<GameOver>>>,
    existing_menu_items_query: Query<Entity, With<MenuType>>,
) {
    let menu_type = if let Some(menu_type) = reader.iter().last() {
        menu_type
    } else {
        return;
    };
    for entity in existing_menu_items_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    let mut menu_creator = MenuCreator {
        commands: &mut commands,
        top: 200.0,
        left: 100.0,
        padding: Vec2::new(10.0, 10.0),
        size: Vec2::new(150.0, 65.0),
        font: font_assets.fira_sans.clone(),
        default_focus: true,
        menu_type: menu_type.clone(),
    };
    match menu_type {
        MenuType::Main => {
            menu_creator.button(0, 0, "Start", MenuAction::StartGame);
            menu_creator.button(0, 1, "Exit", MenuAction::ExitGame);
        }
        MenuType::Pause => {
            menu_creator.button(0, 0, "Resume", MenuAction::ResumeGame);
            menu_creator.button(0, 1, "Main Menu", MenuAction::BackToMainMenu);
            menu_creator.button(0, 2, "Exit", MenuAction::ExitGame);
        }
        MenuType::GameOver(game_over_reason) => {
            if state.current() != &AppState::Game {
                return;
            }
            game_over_state.set(Some(game_over_reason.clone())).unwrap();
            menu_creator.button(0, 0, "Main Menu", MenuAction::BackToMainMenu);
            menu_creator.button(0, 1, "Exit", MenuAction::ExitGame);

            match game_over_reason {
                GameOver::Injured | GameOver::Disqualified => {
                    let caption = match game_over_reason {
                        GameOver::Disqualified => "DISQUALIFIED!",
                        GameOver::Injured => "INJURED!",
                        _ => panic!(),
                    };
                    menu_creator.text(1, 0, caption, Color::RED);
                }
                GameOver::WrongWay => {
                    menu_creator.text(1, 0, "that's the wrong way...", Color::RED);
                }
                GameOver::FinishLine => {
                    menu_creator.text(1, 0, "FINISH!", Color::GREEN);
                }
            }
        }
    }
    if state.current() != &AppState::Menu {
        state.set(AppState::Menu).unwrap();
    }
}

struct MenuCreator<'a, 'w, 's> {
    menu_type: MenuType,
    commands: &'a mut Commands<'w, 's>,
    top: f32,
    left: f32,
    size: Vec2,
    padding: Vec2,
    font: Handle<Font>,
    default_focus: bool,
}

impl MenuCreator<'_, '_, '_> {
    fn button(&mut self, x: i32, y: i32, text: &str, action: MenuAction) -> Entity {
        let text_child = self.text_child(text, Color::BLACK);
        let offset = (self.size + self.padding) * Vec2::new(x as f32, y as f32);
        let mut cmd = self.commands.spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(self.size.x), Val::Px(self.size.y)),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                position: Rect {
                    left: Val::Px(self.left + offset.x),
                    top: Val::Px(self.top + offset.y),
                    ..Default::default()
                },
                ..Default::default()
            },
            color: if false {
                Color::WHITE.into()
            } else {
                Color::GRAY.into()
            },
            ..Default::default()
        });
        cmd.insert(Focusable::default());
        cmd.insert(action);
        cmd.insert(self.menu_type.clone());
        cmd.add_child(text_child);
        if self.default_focus {
            cmd.insert(DefaultFocus);
            self.default_focus = false;
        }
        cmd.id()
    }

    fn text(&mut self, x: i32, y: i32, text: &str, color: Color) -> Entity {
        let text_child = self.text_child(text, color);
        let offset = (self.size + self.padding) * Vec2::new(x as f32, y as f32);
        let mut cmd = self.commands.spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(self.size.x), Val::Px(self.size.y)),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                position: Rect {
                    left: Val::Px(self.left + offset.x),
                    top: Val::Px(self.top + offset.y),
                    ..Default::default()
                },
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        });
        cmd.insert(self.menu_type.clone());

        cmd.add_child(text_child);
        cmd.id()
    }

    fn text_child(&mut self, text: &str, color: Color) -> Entity {
        self.commands
            .spawn_bundle(TextBundle {
                text: Text::with_section(
                    text,
                    TextStyle {
                        font: self.font.clone(),
                        font_size: 30.0,
                        color,
                        ..Default::default()
                    },
                    TextAlignment {
                        vertical: VerticalAlign::Center,
                        horizontal: HorizontalAlign::Center,
                    },
                ),
                ..Default::default()
            })
            .id()
    }
}

#[derive(Component)]
#[component(storage = "SparseSet")]
struct DefaultFocus;

fn focus_default(
    mut commands: Commands,
    focusable_query: Query<&Focusable>,
    default_focus_query: Query<Entity, (With<DefaultFocus>, With<Focusable>)>,
    mut focus_writer: EventWriter<NavRequest>,
) {
    if !focusable_query
        .iter()
        .any(|focusable| focusable.state() == FocusState::Focused)
    {
        for entity in default_focus_query.iter() {
            focus_writer.send(NavRequest::FocusOn(entity));
            commands.entity(entity).remove::<DefaultFocus>();
        }
    }
}

fn handle_nav_events(
    mut reader: EventReader<NavEvent>,
    mut colors: Query<&mut UiColor>,
    menu_actions: Query<&MenuAction>,
    mut action_writer: EventWriter<MenuAction>,
) {
    for event in reader.iter() {
        match event {
            NavEvent::FocusChanged { from, to } => {
                let _ = colors.get_mut(*from.first()).map(|mut color| {
                    color.0 = Color::GRAY;
                });
                let _ = colors.get_mut(*to.first()).map(|mut color| {
                    color.0 = Color::WHITE;
                });
            }
            NavEvent::NoChanges { request, from } => match request {
                NavRequest::Action => {
                    if let Ok(action) = menu_actions.get(*from.first()) {
                        action_writer.send(action.clone());
                    }
                }
                NavRequest::FocusOn(entity) => {
                    let _ = colors.get_mut(*entity).map(|mut color| {
                        color.0 = Color::WHITE;
                    });
                }
                _ => {}
            },
            NavEvent::Locked(_) => {}
            NavEvent::Unlocked(_) => {}
        }
    }
}

fn handle_menu_actions(
    mut reader: EventReader<MenuAction>,
    mut state: ResMut<State<AppState>>,
    mut exit: EventWriter<bevy::app::AppExit>,
    mut menu_writer: EventWriter<MenuType>,
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
                menu_writer.send(MenuType::Main);
            }
        }
    }
}

fn destroy_menu_on_stage_exit(
    mut commands: Commands,
    existing_menu_items_query: Query<Entity, With<MenuType>>,
) {
    for entity in existing_menu_items_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn pause_unpause_game(
    input_query: Query<&InputView<UiBinding>>,
    state: Res<State<AppState>>,
    mut menu_writer: EventWriter<MenuType>,
    mut action_writer: EventWriter<MenuAction>,
    menu_type_query: Query<&MenuType>,
) {
    if !input_query
        .iter()
        .any(|input_view| input_view.key(&UiBinding::Pause).just_pressed())
    {
        return;
    }
    match state.current() {
        AppState::Menu => {
            if menu_type_query
                .iter()
                .any(|menu_type| matches!(menu_type, MenuType::Pause))
            {
                action_writer.send(MenuAction::ResumeGame);
            }
        }
        AppState::ClearLevelAndThenLoad => {}
        AppState::LoadLevel => {}
        AppState::Game => {
            menu_writer.send(MenuType::Pause);
        }
    }
}
