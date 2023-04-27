use bevy::prelude::*;
use bevy::time::Stopwatch;

#[derive(Component)]
pub struct Player {
    pub handle: usize,
}

#[derive(Resource)]
pub struct LocalPlayer(pub usize);

#[derive(States, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub enum GameState {
    #[default]
    Match,
    Ready,
    Game,
    GameOver,
}

#[derive(Resource)]
pub struct GameDuration {
    pub game_time: Stopwatch,
}

#[derive(Resource)]
pub struct Scoreboard {
    pub score: usize,
}