use bevy::prelude::*;
use bevy::time::Stopwatch;

#[derive(Component)]
pub struct Player {
    pub handle: usize,
}

// Player_src는 Player의 상태를 변경하는 구조체
// 위의 Player Component 구조체의 데이터로 넣지 않은 이유는 mut 과정에서 충돌이 나기 때문..
#[derive(Component)]
pub struct PlayerSrc{
    pub score: usize,
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