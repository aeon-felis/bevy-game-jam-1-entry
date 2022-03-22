use bevy::prelude::*;
use ezinput::prelude::*;

use crate::global_types::InputBinding;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EZInputPlugin::<InputBinding>::default());

        app.add_startup_system(setup_input);
    }
}

fn setup_input(mut commands: Commands) {
    let mut view = InputView::empty();

    let mut binding = ActionBinding::from(InputBinding::Rotate);
    for (input, axis_value) in [
        (BindingInputReceiver::KeyboardKey(KeyCode::Left), 1.0),
        (BindingInputReceiver::KeyboardKey(KeyCode::A), 1.0),
        (BindingInputReceiver::KeyboardKey(KeyCode::Right), -1.0),
        (BindingInputReceiver::KeyboardKey(KeyCode::D), -1.0),
    ] {
        binding.receiver(input).default_axis_value(input, axis_value);
    }
    view.add_binding(&binding);

    view.add_binding({
        ActionBinding::from(InputBinding::Pause)
            .receiver(BindingInputReceiver::KeyboardKey(KeyCode::Escape))
    });

    commands.spawn()
        .insert(view)
        .insert(EZInputKeyboardService);
}
