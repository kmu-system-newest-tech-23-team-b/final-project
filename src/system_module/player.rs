use bevy::prelude::*;
use bevy_ggrs::{ggrs, PlayerInputs};

use crate::component::Player;
use crate::system_module::network::GgrsConfig;

const UP: u8 = 1 << 0;
const DOWN: u8 = 1 << 1;
const LEFT: u8 = 1 << 2;
const RIGHT: u8 = 1 << 3;

pub fn input(_: In<ggrs::PlayerHandle>, keys: Res<Input<KeyCode>>) -> u8 {
    let mut input = 0;
    if keys.any_pressed([KeyCode::Up]) { input |= UP; }
    if keys.any_pressed([KeyCode::Down]) { input |= DOWN; }
    if keys.any_pressed([KeyCode::Left]) { input |= LEFT }
    if keys.any_pressed([KeyCode::Right]) { input |= RIGHT; }
    input
}

pub fn move_system(pi: Res<PlayerInputs<GgrsConfig>>, mut query: Query<(&mut Transform, &Player)>) {
    for (mut transform, player) in query.iter_mut() {
        let (input, _) = pi[player.handle];
        let mut direction = Vec2::ZERO;
        if input & UP != 0 { direction.y += 1.; }
        if input & DOWN != 0 { direction.y -= 1.; }
        if input & RIGHT != 0 { direction.x += 1.; }
        if input & LEFT != 0 { direction.x -= 1.; }
        if direction == Vec2::ZERO { continue; }
        transform.translation += (direction * 0.1).extend(0.);
    }
}