use bevy::time::Stopwatch;
use uuid::Uuid;
use bevy::{prelude::*};
use bevy_asset_loader::prelude::*;
use bevy_ggrs::*;
use bevy_matchbox::prelude::*;
use bevy::math::Vec3Swizzles;

use crate::component::{GameState, GameDuration, Playerid};
use crate::system_module::network::{GgrsConfig, wait_socket};
use crate::system_module::player::{input, move_system, transition_state, enemy_movement};
use crate::system_module::score::{update_game_data};
use crate::system_module::startup::{play_music, setup, set_player, set_time_score, spawn_enemy, despawn_enemy, game_start_ui, despawn_start_ui};
use crate::system_module::view::follow;
use game_ui::GameOverPlugin;

use crate::component::Player;
use crate::component::Enemy;

mod system_module;
mod component;
mod game_ui;

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
        .add_plugin(GameOverPlugin)
        .insert_resource(ClearColor(Color::WHITE))
        .insert_resource(GameDuration { game_time: Stopwatch::new() })
        .insert_resource(Playerid { id_local: Uuid::new_v4(), id_remote: Uuid::new_v4() })
        .insert_resource(MatchboxSocket::new_ggrs("ws://127.0.0.1:3536/room"))
        .add_systems((
            play_music.in_schedule(OnEnter(GameState::Match)),
            setup.in_schedule(OnEnter(GameState::Match)),
            wait_socket.run_if(in_state(GameState::Match)),
            transition_state.in_schedule(GGRSSchedule),

            // Matching and Ready
            set_time_score.in_schedule(OnEnter(GameState::Ready)),
            set_player.in_schedule(OnEnter(GameState::Ready)),
            follow.run_if(in_state(GameState::Ready)),
            game_start_ui.in_schedule(OnEnter(GameState::Ready)),
            despawn_start_ui.in_schedule(OnExit(GameState::Ready)),

            // Start Game
            move_system.in_schedule(GGRSSchedule).run_if(in_state(GameState::Game)),
            update_game_data.run_if(in_state(GameState::Game)),
            follow.run_if(in_state(GameState::Game)),
            spawn_enemy.in_schedule(OnEnter(GameState::Game)),
            
        ))
        .add_systems((
            enemy_movement,
            despawn_enemy,
        ))
        .add_system(enemy_hit_player)
        .run();
}

pub fn enemy_hit_player(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
) {
    for player_transform in player_query.iter() {
        for enemy_transform in enemy_query.iter() {
            let distance = Vec2::distance(player_transform.translation.xy(), enemy_transform.translation.xy(),);
            if distance < 0.3 { commands.insert_resource(NextState(Some(GameState::GameOver)));}
        }
    }
}