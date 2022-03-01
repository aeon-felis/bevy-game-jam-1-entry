use bevy::prelude::*;

#[derive(Hash, Debug, PartialEq, Eq, Clone)]
pub enum AppState {
    Menu,
    ClearLevelAndThenLoad,
    LoadLevel,
    Game,
}

#[derive(Hash, Debug, PartialEq, Eq, Clone)]
pub enum GameOver {
    Injured,
    Disqualified,
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

#[derive(Component)]
pub struct DespawnWithLevel;

#[derive(Component)]
pub struct CameraFollowTarget;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerHead;

#[derive(Component)]
pub struct Hurdle;

#[derive(Component)]
pub struct Ground;
