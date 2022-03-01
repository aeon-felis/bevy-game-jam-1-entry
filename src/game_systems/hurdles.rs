use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::ops::Range;

use rand::prelude::SliceRandom;
use rand::Rng;

use crate::consts::{BEFORE_FIRST, HURDLE_HEIGHT, HURDLE_SPACING, HURDLE_WIDTH};
use crate::global_types::{AppState, DespawnWithLevel, GameBoundaries, GameOver, Hurdle, Player};
use crate::ui::MenuType;
use crate::utils::entities_ordered_by_type;

pub struct HurdlesPlugin;

impl Plugin for HurdlesPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::LoadLevel).with_system(add_hurdles));
        app.add_system_set(SystemSet::on_update(AppState::Game).with_system(detect_hurdle_touch));
    }
}

fn distribute_distances(max_num: usize, over_range: Range<f32>, min_size: f32) -> Vec<f32> {
    if max_num == 0 {
        return Vec::new();
    }
    let mut distances = vec![over_range.end - over_range.start];
    for _ in 0..max_num {
        let (current_max_pos, _) = distances
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .unwrap();
        let to_cut = distances.remove(current_max_pos);
        if to_cut < min_size * 2.0 {
            distances.push(to_cut);
            break;
        }
        let cut_range = min_size..(to_cut - min_size);
        let cut_at = if cut_range.is_empty() {
            min_size
        } else {
            rand::thread_rng().gen_range(cut_range)
        };
        distances.push(cut_at);
        distances.push(to_cut - cut_at);
    }
    distances.shuffle(&mut rand::thread_rng());
    let mut so_far = over_range.start;
    let num_to_return = distances.len() - 1;
    distances
        .into_iter()
        .take(num_to_return)
        .map(|distance| {
            so_far += distance;
            so_far
        })
        .collect()
}

fn add_hurdles(mut commands: Commands, game_boundaries: Res<GameBoundaries>) {
    let allowed_width = game_boundaries.right - BEFORE_FIRST;
    let placements = distribute_distances(
        (allowed_width / HURDLE_SPACING) as usize,
        BEFORE_FIRST..game_boundaries.right,
        HURDLE_SPACING,
    );

    for placement in placements {
        let mut cmd = commands.spawn_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Dynamic.into(),
            position: point![placement, HURDLE_HEIGHT * 0.5].into(),
            ..Default::default()
        });
        cmd.insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(HURDLE_WIDTH * 0.5, HURDLE_HEIGHT * 0.5).into(),
            ..Default::default()
        });
        cmd.insert(ColliderDebugRender::with_id(5));
        cmd.insert(ColliderPositionSync::Discrete);
        cmd.insert(DespawnWithLevel);
        cmd.insert(Hurdle);
    }
}

fn detect_hurdle_touch(
    mut reader: EventReader<ContactEvent>,
    hurdle_query: Query<(), With<Hurdle>>,
    player_query: Query<(), With<Player>>,
    mut menu_writer: EventWriter<MenuType>,
) {
    for event in reader.iter() {
        if let ContactEvent::Started(handle1, handle2) = event {
            if entities_ordered_by_type!(
                [handle1.entity(), handle2.entity()],
                hurdle_query,
                player_query
            )
            .is_some()
            {
                menu_writer.send(MenuType::GameOver(GameOver::Disqualified));
            }
        }
    }
}
