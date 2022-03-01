use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use ezinput::prelude::{
    ActionBinding, AxisState, BindingInputReceiver, BindingTypeView, InputView, PressState,
};

use crate::global_types::{AppState, CameraFollowTarget, DespawnWithLevel, Player, PlayerHead, GameBoundaries, GameOver};
use crate::ui::MenuType;

pub struct PogoPlugin;

impl Plugin for PogoPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::LoadLevel).with_system(spawn_player));
        app.add_plugin(ezinput::prelude::EZInputPlugin::<ControlBinding>::default());
        app.add_system_set({
            SystemSet::on_update(AppState::Game)
                .with_system(player_controls)
                .with_system(automatically_balance_player)
                .with_system(detect_out_of_bounds)
        });
    }
}

#[derive(ezinput_macros::BindingTypeView, PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum ControlBinding {
    Spin,
}

#[derive(Component)]
struct AutoBalance;

fn spawn_player(mut commands: Commands) {
    let mut body_cmd = commands.spawn_bundle(RigidBodyBundle {
        body_type: RigidBodyType::Dynamic.into(),
        position: point![0.0, 4.0].into(),
        mass_properties: MassProperties {
            local_com: point![0.0, 1.0],
            inv_mass: 1.0,
            inv_principal_inertia_sqrt: 1.0,
        }
        .into(),
        ..Default::default()
    });
    body_cmd.insert_bundle(ColliderBundle {
        shape: ColliderShape::cuboid(0.5, 0.5).into(),
        flags: ColliderFlags {
            active_events: ActiveEvents::CONTACT_EVENTS,
            ..Default::default()
        }
        .into(),
        position: Vec2::new(0.0, 0.5).into(),
        ..Default::default()
    });
    body_cmd.insert(ColliderDebugRender::with_id(2));
    body_cmd.insert(ColliderPositionSync::Discrete);
    body_cmd.insert(AutoBalance);
    body_cmd.insert(CameraFollowTarget);
    body_cmd.insert(DespawnWithLevel);
    body_cmd.insert(Player);
    body_cmd.insert(PlayerHead);

    let mut view = InputView::empty();
    view.add_binding(ControlBinding::Spin, &{
        let mut binding = ActionBinding::from(ControlBinding::Spin);

        for (binding_input_receiver, axis_value) in [
            (BindingInputReceiver::KeyboardKey(KeyCode::Left), -1.0),
            (BindingInputReceiver::KeyboardKey(KeyCode::Right), 1.0),
        ] {
            binding.receiver(binding_input_receiver);
            binding.default_axis_value(binding_input_receiver, axis_value);
        }

        binding.receiver(BindingInputReceiver::GamepadAxis(
            GamepadAxisType::LeftStickX,
        ));

        binding
    });
    body_cmd.insert(view);
    body_cmd.insert(ezinput::prelude::EZInputKeyboardService::default());
    body_cmd.insert(ezinput::prelude::EZInputGamepadService::default());

    let body_entity = body_cmd.id();

    let mut stick_cmd = commands.spawn();
    stick_cmd.insert(ColliderParentComponent(ColliderParent {
        handle: body_entity.handle(),
        pos_wrt_parent: Vec2::new(0.0, 0.0).into(),
    }));
    stick_cmd.insert_bundle(ColliderBundle {
        shape: ColliderShape::cuboid(0.1, 0.5).into(),
        flags: ColliderFlags {
            active_events: ActiveEvents::CONTACT_EVENTS,
            ..Default::default()
        }
        .into(),
        material: ColliderMaterial {
            restitution: 2.0,
            friction: 1.0,
            ..Default::default()
        }
        .into(),
        ..Default::default()
    });
    stick_cmd.insert(ColliderDebugRender::with_id(3));
    stick_cmd.insert(ColliderPositionSync::Discrete);
    stick_cmd.insert(DespawnWithLevel);
    stick_cmd.insert(Player);
}

fn player_controls(
    time: Res<Time>,
    mut query: Query<(
        &InputView<ControlBinding>,
        &mut RigidBodyVelocityComponent,
        &RigidBodyMassPropsComponent,
    )>,
) {
    let torque = time.delta().as_secs_f32() * 10.0;
    for (input_view, mut velocity, mass_props) in query.iter_mut() {
        if let Some(AxisState(spin_axis_value, PressState::Pressed { .. })) =
            input_view.axis(&ControlBinding::Spin).first()
        {
            velocity.apply_torque_impulse(mass_props, torque * -*spin_axis_value);
        }
    }
}

fn automatically_balance_player(
    time: Res<Time>,
    mut query: Query<
        (
            &RigidBodyPositionComponent,
            &mut RigidBodyVelocityComponent,
            &RigidBodyMassPropsComponent,
        ),
        With<AutoBalance>,
    >,
) {
    let torque = time.delta().as_secs_f32() * 2.0;
    for (position, mut velocity, mass_props) in query.iter_mut() {
        let angle = position.0.position.rotation.angle();
        velocity.apply_torque_impulse(mass_props, torque * -angle);
    }
}

fn detect_out_of_bounds(
    game_over_state: Res<State<Option<GameOver>>>,
    player_query: Query<&RigidBodyPositionComponent, With<PlayerHead>>,
    mut menu_writer: EventWriter<MenuType>,
    game_boundaries: Res<GameBoundaries>,
) {
    if game_over_state.current().is_some() {
        return;
    }
    for player_position in player_query.iter() {
        // TODO: Once I add the sprite I should be able to just use the GlobalTransform
        let player_position = player_position.position.translation.x;
        if player_position < game_boundaries.left {
            menu_writer.send(MenuType::GameOver(GameOver::WrongWay));
        } else if game_boundaries.right < player_position {
            menu_writer.send(MenuType::GameOver(GameOver::FinishLine));
        }
    }
}
