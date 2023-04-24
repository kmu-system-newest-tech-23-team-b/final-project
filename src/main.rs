use bevy::{prelude::*};
use bevy_asset_loader::prelude::*;
use bevy_ggrs::*;
use bevy_matchbox::prelude::*;

use crate::component::{GameState, Scoreboard};
use crate::system_module::network::{GgrsConfig, wait_socket};
use crate::system_module::player::{input, move_system};
use crate::system_module::score::update_score;
use crate::system_module::startup::{setup, spawn};
use crate::system_module::view::follow;

use crate::component::Player;
use crate::component::Enemy;

pub const PLAYER_SIZE: f32 = 64.0;
pub const ENEMY_SIZE: f32 = 64.0;

mod system_module;
mod component;

fn main() {
    let mut app = App::new();
    app.add_state::<GameState>().add_loading_state(
        LoadingState::new(GameState::Match)
    );
    GGRSPlugin::<GgrsConfig>::new().with_input_system(input)
        .register_rollback_component::<Transform>().build(&mut app);
    app.insert_resource(ClearColor(Color::WHITE))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Final Project Team B".to_string(),
                fit_canvas_to_parent: true,
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::WHITE))
        .insert_resource(Scoreboard { score: 0 })
        .insert_resource(MatchboxSocket::new_ggrs("ws://127.0.0.1:3536/room"))
        .add_systems((
            setup.in_schedule(OnEnter(GameState::Match)),
            wait_socket.run_if(in_state(GameState::Match)),
            spawn.in_schedule(OnEnter(GameState::Game)),
            follow.run_if(in_state(GameState::Game)),
            update_score.run_if(in_state(GameState::Game)),
        ))
        .add_systems((move_system.in_schedule(GGRSSchedule), ))
        .add_system(enemy_hit_player)
        .run();
}

pub fn enemy_hit_player(
    mut player_query: Query<&Transform, With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
) {
    if let Ok(player_transform) = player_query.get_single_mut() {
        for enemy_transform in enemy_query.iter() {
            let distance = player_transform.translation.distance(enemy_transform.translation);
            let player_radius = PLAYER_SIZE / 2.0;
            let enemy_radius = ENEMY_SIZE / 2.0;
            if distance < player_radius + enemy_radius {
                println!("Enemy Hit Player!");
            }
        }
    }
}