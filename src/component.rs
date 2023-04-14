use bevy::prelude::*;

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
    Game,
}