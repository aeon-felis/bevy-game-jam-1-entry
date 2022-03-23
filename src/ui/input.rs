use bevy::prelude::*;
use ezinput::prelude::*;

use crate::global_types::InputBinding;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EZInputPlugin::<InputBinding>::default());

        app.add_startup_system(setup_input);
        app.add_system(handle_gamepad_events);
    }
}

fn create_view() -> InputView<InputBinding> {
    let mut view = InputView::empty();

    let mut binding = ActionBinding::from(InputBinding::Rotate);
    for (input, axis_value) in [
        (BindingInputReceiver::KeyboardKey(KeyCode::Left), -1.0),
        (BindingInputReceiver::KeyboardKey(KeyCode::A), -1.0),
        (BindingInputReceiver::KeyboardKey(KeyCode::Right), 1.0),
        (BindingInputReceiver::KeyboardKey(KeyCode::D), 1.0),
        (BindingInputReceiver::GamepadButton(GamepadButtonType::DPadLeft), -1.0),
        (BindingInputReceiver::GamepadButton(GamepadButtonType::DPadRight), 1.0),
    ] {
        binding.receiver(input).default_axis_value(input, axis_value);
    }
    binding.receiver(BindingInputReceiver::GamepadAxis(GamepadAxisType::LeftStickX));
    binding.receiver(BindingInputReceiver::GamepadAxis(GamepadAxisType::DPadX));
    view.add_binding(&binding);

    view.add_binding({
        ActionBinding::from(InputBinding::Pause)
            .receiver(BindingInputReceiver::KeyboardKey(KeyCode::Escape))
            .receiver(BindingInputReceiver::GamepadButton(GamepadButtonType::Start))
    });

    view
}

fn setup_input(mut commands: Commands) {

    commands.spawn()
        .insert(create_view())
        .insert(EZInputKeyboardService);
}

fn handle_gamepad_events(
    mut reader: EventReader<GamepadEvent>,
    gamepad_services: Query<(Entity, &EZInputGamepadService), With<InputView<InputBinding>>>,
    mut commands: Commands,
) {
    for GamepadEvent(gamepad, event_type) in reader.iter() {
        match event_type {
            GamepadEventType::Connected => {
                if !gamepad_services.iter().any(|(_, service)| service.0 == *gamepad) {
                    commands.spawn()
                        .insert(create_view())
                        .insert(EZInputGamepadService(*gamepad));
                }
            }
            GamepadEventType::Disconnected => {
                for (entity, service) in gamepad_services.iter() {
                    if service.0 == *gamepad {
                        commands.entity(entity).despawn();
                    }
                }
            }
            _ => {}
        }
    }
}
