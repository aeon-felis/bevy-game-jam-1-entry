use bevy::prelude::*;

use crate::components::CameraFollowTarget;
use crate::AppState;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_camera);
        app.add_system_set(SystemSet::on_enter(AppState::LoadLevel).with_system(reset_camera));
        app.add_system(set_camera_target);
        app.add_system(handle_camera_movement);
    }
}

fn setup_camera(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.transform.scale.x *= 0.02;
    camera.transform.scale.y *= 0.02;
    camera.transform.translation.y = 2.0;
    commands.spawn_bundle(camera).insert(CameraBehavior {
        target: 0.0,
        velocity: 0.0,
    });
}

fn reset_camera(mut query: Query<(&mut CameraBehavior, &mut Transform)>) {
    for (mut camera_behavior, mut camera_transform) in query.iter_mut() {
        *camera_behavior = CameraBehavior {
            target: 0.0,
            velocity: 0.0,
        };
        camera_transform.translation.x = 0.0;
    }
}

#[derive(Component)]
struct CameraBehavior {
    target: f32,
    velocity: f32,
}

fn set_camera_target(
    mut camera_query: Query<&mut CameraBehavior>,
    target_query: Query<
        &bevy_rapier2d::prelude::RigidBodyPositionComponent,
        With<CameraFollowTarget>,
    >,
) {
    for mut camera in camera_query.iter_mut() {
        for target in target_query.iter() {
            // TODO: Once I add the sprite I should be able to just use the GlobalTransform
            let target = target.position.translation;
            camera.target = target.x;
        }
    }
}

fn handle_camera_movement(
    time: Res<Time>,
    mut query: Query<(&mut CameraBehavior, &mut Transform)>,
) {
    let duration = time.delta().as_secs_f32();
    let acceleration = duration * 10.0;
    for (mut camera_behavior, mut camera_transform) in query.iter_mut() {
        let to_move = camera_behavior.target - camera_transform.translation.x;
        let target_velocity = if 2.0 < to_move.abs() {
            to_move.signum() * 50.0
        } else {
            0.0
        };
        let target_acceleration = target_velocity - camera_behavior.velocity;
        camera_behavior.velocity += target_acceleration.signum() * acceleration;
        camera_transform.translation.x += camera_behavior.velocity * duration;
    }
}
