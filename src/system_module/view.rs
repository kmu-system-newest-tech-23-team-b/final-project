use bevy::prelude::*;

use crate::component::{LocalPlayer, Player};

pub const MAP_SIZE: u32 = 10;

pub fn follow(player: Option<Res<LocalPlayer>>, query: Query<(&Player, &Transform)>,
              mut camera: Query<&mut Transform, (With<Camera>, Without<Player>)>, ) {
    let is_handle = match player {
        Some(handle) => handle.0,
        None => return
    };
    for (player, player_transform) in query.iter() {
        if player.handle != is_handle { continue; }
        let pos = player_transform.translation;
        for mut transform in camera.iter_mut() {
            transform.translation.x = pos.x;
            transform.translation.y = pos.y;
        }
    }
}

