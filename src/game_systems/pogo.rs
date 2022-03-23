use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use ezinput::prelude::*;

use crate::global_types::{
    AppState, CameraFollowTarget, DespawnWithLevel, GameBoundaries, GameOver, MenuState, Player,
    PlayerHead, PlayerSprite, PlayerStatus, InputBinding,
};
use crate::loading::TextureAssets;

pub struct PogoPlugin;

impl Plugin for PogoPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::LoadLevel).with_system(spawn_player));
        app.add_system_set({
            SystemSet::on_update(AppState::Game)
                .with_system(player_controls)
                .with_system(automatically_balance_player)
                .with_system(detect_out_of_bounds)
                .with_system(update_player_status)
        });
    }
}

#[derive(Component)]
struct AutoBalance;

fn spawn_player(mut commands: Commands, texture_assets: Res<TextureAssets>) {
    let mut player_cmd = commands.spawn_bundle(RigidBodyBundle {
        body_type: RigidBodyType::Dynamic.into(),
        position: point![0.0, 4.0].into(),
        mass_properties: MassProperties {
            local_com: point![0.0, 1.0],
            inv_mass: 1.0,
            inv_principal_inertia_sqrt: 1.0,
        }
        .into(),
        damping: RigidBodyDamping {
            linear_damping: 0.0,
            angular_damping: 1.0,
        }
        .into(),
        ..Default::default()
    });
    player_cmd.insert_bundle(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(2.0, 2.0)),
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 0.1),
            ..Default::default()
        },
        texture: texture_assets.pogo_player.clone(),
        ..Default::default()
    });
    player_cmd.insert(DespawnWithLevel);
    player_cmd.insert(AutoBalance);
    player_cmd.insert(RigidBodyPositionSync::Discrete);
    player_cmd.insert(CameraFollowTarget);
    player_cmd.insert(PlayerSprite);

    let player_entity = player_cmd.id();

    let mut body_cmd = commands.spawn();
    body_cmd.insert(ColliderParentComponent(ColliderParent {
        handle: player_entity.handle(),
        pos_wrt_parent: Vec2::new(0.0, 0.25).into(),
    }));
    body_cmd.insert_bundle(ColliderBundle {
        shape: ColliderShape::cuboid(0.5, 0.75).into(),
        flags: ColliderFlags {
            active_events: ActiveEvents::CONTACT_EVENTS,
            ..Default::default()
        }
        .into(),
        position: Vec2::new(0.0, 1.0).into(),
        ..Default::default()
    });
    body_cmd.insert(Player);
    body_cmd.insert(PlayerHead);

    let mut stick_cmd = commands.spawn();
    stick_cmd.insert(ColliderParentComponent(ColliderParent {
        handle: player_entity.handle(),
        pos_wrt_parent: Vec2::new(0.0, -0.75).into(),
    }));
    stick_cmd.insert_bundle(ColliderBundle {
        shape: ColliderShape::cuboid(0.1, 0.25).into(),
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
    stick_cmd.insert(DespawnWithLevel);
    stick_cmd.insert(Player);
}

fn player_controls(
    time: Res<Time>,
    input_views: Query<&InputView<InputBinding>>,
    mut query: Query<
        (
            &mut RigidBodyVelocityComponent,
            &RigidBodyMassPropsComponent,
        ),
        With<PlayerSprite>,
    >,
) {
    let mut spin_value = 0.0;
    let mut num_participating = 0;
    for input_view in input_views.iter() {
        for axis_value in input_view.axis(&InputBinding::Rotate) {
            if !axis_value.1.released() {
                num_participating += 1;
                spin_value -= axis_value.0
            }
        }
    }
    if 0 < num_participating {
        spin_value /= num_participating as f32;
        let torque = time.delta().as_secs_f32() * 30.0 * spin_value;
        for (mut velocity, mass_props) in query.iter_mut() {
            velocity.apply_torque_impulse(mass_props, torque);
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
    let torque = time.delta().as_secs_f32() * 20.0;
    for (position, mut velocity, mass_props) in query.iter_mut() {
        let angle = position.0.position.rotation.angle();
        if 0.2 <= angle.abs() {
            velocity.apply_torque_impulse(mass_props, torque * -angle.clamp(-1.0, 1.0));
        }
    }
}

fn detect_out_of_bounds(
    player_query: Query<&GlobalTransform, With<PlayerSprite>>,
    game_boundaries: Res<GameBoundaries>,
    mut state: ResMut<State<AppState>>,
    mut game_over_state: ResMut<State<Option<GameOver>>>,
) {
    for player_position in player_query.iter() {
        let player_position = player_position.translation.x;
        if player_position < game_boundaries.left {
            state.set(AppState::Menu(MenuState::GameOver)).unwrap();
            game_over_state.set(Some(GameOver::WrongWay)).unwrap();
        } else if game_boundaries.right < player_position {
            state.set(AppState::Menu(MenuState::GameOver)).unwrap();
            game_over_state.set(Some(GameOver::FinishLine)).unwrap();
        }
    }
}

fn update_player_status(
    time: Res<Time>,
    player_query: Query<&GlobalTransform, With<PlayerSprite>>,
    mut player_status: ResMut<PlayerStatus>,
) {
    for player_position in player_query.iter() {
        player_status.distance_traveled = player_position.translation.x;
    }
    player_status.time += time.delta();
}
