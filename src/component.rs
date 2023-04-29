use bevy::prelude::*;
use std::rc::Rc;

#[derive(Component)]
pub struct Player {
    pub handle: usize,
}

#[derive(Component)]
pub struct Enemy{
    pub handle: usize,
    pub position: Vec2,
    pub speed: f32,
}


#[derive(Resource)]
pub struct LocalPlayer(pub usize);

#[derive(States, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub enum GameState {
    #[default]
    Match,
    Game,
}

#[derive(Resource)]
pub struct Scoreboard {
    pub score: usize,
}