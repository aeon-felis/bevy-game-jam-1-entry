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

    view.add_binding({
        ActionBinding::from(InputBinding::Rotate)
            .receiver(BindingInputReceiver::KeyboardKey(KeyCode::Left))
            .default_axis_value(BindingInputReceiver::KeyboardKey(KeyCode::Left), 1.0)
            .receiver(BindingInputReceiver::KeyboardKey(KeyCode::Right))
            .default_axis_value(BindingInputReceiver::KeyboardKey(KeyCode::Right), -1.0)
    });

    view.add_binding({
        ActionBinding::from(InputBinding::Pause)
            .receiver(BindingInputReceiver::KeyboardKey(KeyCode::Escape))
    });

    commands.spawn()
        .insert(view)
        .insert(EZInputKeyboardService);
}
