use bevy::prelude::*;
use bevy_ui_navigation::systems::InputMapping;
use bevy_ui_navigation::{FocusState, Focusable, NavEvent, NavRequest};

use crate::loading::FontAssets;
use crate::AppState;

pub struct UiPlugin;

#[derive(Component, Clone)]
enum MenuType {
    Main,
    Pause,
}

#[derive(Component, Clone)]
enum MenuAction {
    StartGame,
    ResumeGame,
    BackToMainMenu,
    ExitGame,
}

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_ui);
        app.init_resource::<InputMapping>();
        app.add_event::<MenuType>();
        app.add_event::<MenuAction>();
        app.add_system(spawn_menu);
        app.add_startup_system(|mut writer: EventWriter<MenuType>| {
            writer.send(MenuType::Main);
        });
        app.add_system_set({
            SystemSet::on_update(AppState::Menu)
                .with_system(bevy_ui_navigation::systems::default_keyboard_input)
                .with_system(bevy_ui_navigation::systems::default_mouse_input)
                .with_system(bevy_ui_navigation::systems::default_gamepad_input)
        });
        app.add_system(focus_default);
        app.add_system(handle_nav_events);
        app.add_system(handle_menu_actions);
        app.add_system_set(
            SystemSet::on_exit(AppState::Menu).with_system(destroy_menu_on_stage_exit),
        );
        app.add_system_set(SystemSet::on_update(AppState::Game).with_system(pause_game));
        app.add_system_set(SystemSet::on_update(AppState::Menu).with_system(unpause_game));
    }
}

fn setup_ui(mut commands: Commands) {
    commands.spawn_bundle(UiCameraBundle::default());
}

fn spawn_menu(
    mut reader: EventReader<MenuType>,
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    mut state: ResMut<State<AppState>>,
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
        left: 300.0,
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
        // .insert(role.clone())
        cmd.with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    text,
                    TextStyle {
                        font: self.font.clone(),
                        font_size: 30.0,
                        color: Color::BLACK,
                        ..Default::default()
                    },
                    TextAlignment {
                        vertical: VerticalAlign::Center,
                        horizontal: HorizontalAlign::Center,
                    },
                ),
                ..Default::default()
            });
        });
        if self.default_focus {
            cmd.insert(DefaultFocus);
            self.default_focus = false;
        }
        cmd.id()
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
                state.set(AppState::Game).unwrap();
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

fn pause_game(mut keyboard_input: ResMut<Input<KeyCode>>, mut menu_writer: EventWriter<MenuType>) {
    if keyboard_input.pressed(KeyCode::Escape) {
        keyboard_input.reset(KeyCode::Escape);
        menu_writer.send(MenuType::Pause);
    }
}

fn unpause_game(
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut action_writer: EventWriter<MenuAction>,
    menu_type_query: Query<&MenuType>,
) {
    if keyboard_input.pressed(KeyCode::Escape) {
        keyboard_input.reset(KeyCode::Escape);
        if menu_type_query
            .iter()
            .any(|menu_type| matches!(menu_type, MenuType::Pause))
        {
            action_writer.send(MenuAction::ResumeGame);
        }
    }
}
