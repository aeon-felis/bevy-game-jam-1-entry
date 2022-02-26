use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::components::DespawnWithLevel;
use crate::AppState;

pub struct PogoPlugin;

impl Plugin for PogoPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set({ SystemSet::on_enter(AppState::LoadLevel).with_system(spawn_player) });
    }
}

fn spawn_player(mut commands: Commands) {
    let mut body_cmd = commands.spawn_bundle(RigidBodyBundle {
        body_type: RigidBodyType::Dynamic.into(),
        ..Default::default()
    });
    body_cmd.insert_bundle(ColliderBundle {
        shape: ColliderShape::cuboid(0.5, 0.5).into(),
        position: Vec2::new(0.0, 4.5).into(),
        ..Default::default()
    });
    body_cmd.insert(ColliderDebugRender::with_id(2));
    body_cmd.insert(ColliderPositionSync::Discrete);
    body_cmd.insert(DespawnWithLevel);
    let body_entity = body_cmd.id();

    let mut stick_cmd = commands.spawn();
    stick_cmd.insert(ColliderParentComponent(ColliderParent {
        handle: body_entity.handle(),
        pos_wrt_parent: Vec2::new(0.0, 4.0).into(),
    }));
    stick_cmd.insert_bundle(ColliderBundle {
        shape: ColliderShape::cuboid(0.1, 0.5).into(),
        material: ColliderMaterial {
            restitution: 2.0,
            ..Default::default()
        }
        .into(),
        ..Default::default()
    });
    stick_cmd.insert(ColliderDebugRender::with_id(3));
    stick_cmd.insert(ColliderPositionSync::Discrete);
    stick_cmd.insert(DespawnWithLevel);
}
