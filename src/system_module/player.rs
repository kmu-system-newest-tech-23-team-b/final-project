use bevy::prelude::*;

use crate::component::Player;

pub fn move_system(keys: Res<Input<KeyCode>>, mut query: Query<&mut Transform, With<Player>>) {
    let mut direction = Vec2::ZERO;
    if keys.any_pressed([KeyCode::Up]) { direction.y += 1.; }
    if keys.any_pressed([KeyCode::Down]) { direction.y -= 1.; }
    if keys.any_pressed([KeyCode::Right]) { direction.x += 1.; }
    if keys.any_pressed([KeyCode::Left]) { direction.x -= 1.; }
    if direction == Vec2::ZERO { return; }
    for mut transform in query.iter_mut() {
        transform.translation += (direction * 0.1).extend(0.);
    }
}