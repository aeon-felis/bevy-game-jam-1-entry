use std::time::Duration;

use bevy::prelude::*;

#[derive(Hash, Debug, PartialEq, Eq, Clone)]
pub enum AppState {
    Menu(MenuState),
    ClearLevelAndThenLoad,
    LoadLevel,
    Game,
}

#[derive(Hash, Debug, PartialEq, Eq, Clone)]
pub enum MenuState {
    Main,
    Pause,
    GameOver,
}

#[derive(Hash, Debug, PartialEq, Eq, Clone)]
pub enum GameOver {
    Injured,
    Disqualified,
    WrongWay,
    FinishLine,
}

pub struct GameBoundaries {
    pub left: f32,
    pub right: f32,
}

impl GameBoundaries {
    pub fn width(&self) -> f32 {
        self.right - self.left
    }

    pub fn center(&self) -> f32 {
        (self.right + self.left) * 0.5
    }
}

#[derive(Default)]
pub struct PlayerStatus {
    pub distance_traveled: f32,
    pub time: Duration,
    pub competitors_before: usize,
    pub competitors_after: usize,
}

impl PlayerStatus {
    pub fn format_time(&self) -> String {
        let time_in_seconds = self.time.as_secs_f32();
        let only_minutes = time_in_seconds as u32 / 60;
        let only_seconds = time_in_seconds % 60.0;
        format!("{:02}:{:02.1}", only_minutes, only_seconds)
    }

    pub fn place(&self) -> usize {
        self.competitors_before + 1
    }

    pub fn total_runners(&self) -> usize {
        self.competitors_before + self.competitors_after + 1
    }
}

#[derive(Component)]
pub struct DespawnWithLevel;

#[derive(Component)]
pub struct CameraFollowTarget;

#[derive(Component)]
pub struct PlayerSprite;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerHead;

#[derive(Component)]
pub struct Hurdle;

#[derive(Component)]
pub struct Competitor;

#[derive(Component)]
pub struct Ground;
