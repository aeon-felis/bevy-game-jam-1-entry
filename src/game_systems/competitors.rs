use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::global_types::{
    AppState, Competitor, DespawnWithLevel, Hurdle, PlayerSprite, PlayerStatus,
};
use crate::loading::AnimationAssets;

pub struct CompetitorsPlugin;

impl Plugin for CompetitorsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::LoadLevel).with_system(add_competitors));
        app.add_system_set(SystemSet::on_update(AppState::Game).with_system(maintain_speed));
        app.add_system_set(SystemSet::on_update(AppState::Game).with_system(jump_over_hurdles));
        app.add_system_set(SystemSet::on_update(AppState::Game).with_system(update_player_place));
    }
}

#[derive(Component)]
struct MaintainSpeed(f32);

#[derive(Component)]
enum JumpOverNextHurdle {
    LookForHurdleToJumpOver,
    PrepareToJumpOverHurdle(Entity),
    JumpingOverHurdle(Entity),
    PassedAllHurdles,
}

fn add_competitors(mut commands: Commands, animation_assets: Res<AnimationAssets>) {
    for i in 0..4 {
        let mut cmd = commands.spawn();
        cmd.insert_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                custom_size: Some(Vec2::new(2.0, 2.0)),
                ..Default::default()
            }
            .into(),
            texture_atlas: animation_assets.competitor_atlas.clone(),
            ..Default::default()
        });
        cmd.insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Dynamic.into(),
            position: point![2.0 + i as f32 * 3.0, 1.0].into(),
            ..Default::default()
        });
        cmd.insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(0.31, 1.0).into(),
            mass_properties: MassProperties::new(Default::default(), 10.0, 0.0).into(),
            ..Default::default()
        });
        cmd.insert(DespawnWithLevel);
        cmd.insert(RigidBodyPositionSync::Discrete);
        cmd.insert(Competitor);
        cmd.insert(JumpOverNextHurdle::LookForHurdleToJumpOver);
        cmd.insert(MaintainSpeed(3.0 + 0.7 * i as f32));
        cmd.insert(animation_assets.competitor.clone());
        cmd.insert(benimator::Play);
    }
}

fn maintain_speed(
    mut runners_query: Query<(
        &MaintainSpeed,
        &mut RigidBodyVelocityComponent,
        &RigidBodyMassPropsComponent,
    )>,
) {
    for (&MaintainSpeed(target_speed), mut velocity, mass_props) in runners_query.iter_mut() {
        if velocity.linvel.x < target_speed {
            let impulse = Vec2::new(target_speed - velocity.linvel.x, 0.0);
            velocity.apply_impulse(mass_props, impulse.into());
        }
    }
}

fn jump_over_hurdles(
    mut jumpers_query: Query<(
        &mut JumpOverNextHurdle,
        &GlobalTransform,
        &mut RigidBodyVelocityComponent,
        &RigidBodyMassPropsComponent,
    )>,
    hurdles_query: Query<(Entity, &GlobalTransform), With<Hurdle>>,
) {
    for (mut jumper, jumper_transform, mut jumper_velovity, jumper_mass_props) in
        jumpers_query.iter_mut()
    {
        if jumper_transform.translation.x <= 0.1 {
            continue;
        }
        *jumper = match *jumper {
            JumpOverNextHurdle::LookForHurdleToJumpOver => {
                if hurdles_query
                    .iter()
                    .any(|(_, hurdle_transform)| hurdle_transform.translation.x <= 0.1)
                {
                    JumpOverNextHurdle::LookForHurdleToJumpOver
                } else if let Some((hurdle_entity, _)) = hurdles_query
                    .iter()
                    .filter(|(_, hurdle_transform)| {
                        jumper_transform.translation.x < hurdle_transform.translation.x
                    })
                    .min_by(|(_, t1), (_, t2)| {
                        t1.translation.x.partial_cmp(&t2.translation.x).unwrap()
                    })
                {
                    JumpOverNextHurdle::PrepareToJumpOverHurdle(hurdle_entity)
                } else {
                    JumpOverNextHurdle::PassedAllHurdles
                }
            }
            JumpOverNextHurdle::PrepareToJumpOverHurdle(entity) => {
                let hurdle_transform = hurdles_query
                    .get_component::<GlobalTransform>(entity)
                    .unwrap();
                let distance_to_entity =
                    hurdle_transform.translation.x - jumper_transform.translation.x;
                if distance_to_entity < 4.0 {
                    jumper_velovity.apply_impulse(jumper_mass_props, Vec2::new(40.0, 100.0).into());
                    JumpOverNextHurdle::JumpingOverHurdle(entity)
                } else {
                    JumpOverNextHurdle::PrepareToJumpOverHurdle(entity)
                }
            }
            JumpOverNextHurdle::JumpingOverHurdle(entity) => {
                let hurdle_transform = hurdles_query
                    .get_component::<GlobalTransform>(entity)
                    .unwrap();
                let passed_entity_by =
                    jumper_transform.translation.x - hurdle_transform.translation.x;
                if passed_entity_by <= 2.0 {
                    JumpOverNextHurdle::JumpingOverHurdle(entity)
                } else {
                    jumper_velovity
                        .apply_impulse(jumper_mass_props, Vec2::new(-40.0, -100.0).into());
                    JumpOverNextHurdle::LookForHurdleToJumpOver
                }
            }
            JumpOverNextHurdle::PassedAllHurdles => JumpOverNextHurdle::PassedAllHurdles,
        }
    }
}

fn update_player_place(
    competitors_query: Query<&GlobalTransform, With<Competitor>>,
    players_query: Query<&GlobalTransform, With<PlayerSprite>>,
    mut player_status: ResMut<PlayerStatus>,
) {
    for player in players_query.iter() {
        player_status.competitors_before = 0;
        player_status.competitors_after = 0;
        for competitor in competitors_query.iter() {
            if competitor.translation.x < player.translation.x {
                player_status.competitors_after += 1;
            } else {
                player_status.competitors_before += 1;
            }
        }
    }
}
