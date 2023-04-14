use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub handle: usize,
}

#[derive(Resource)]
pub struct LocalPlayer(pub usize);