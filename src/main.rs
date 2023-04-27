use bevy::input::keyboard;
use bevy::time::Stopwatch;
use bevy::transform::commands;
use bevy::{prelude::*};
use bevy_asset_loader::prelude::*;
use bevy_ggrs::*;
use bevy_matchbox::prelude::*;
use system_module::player;

use crate::component::{GameState, GameDuration, Scoreboard};
use crate::system_module::network::{GgrsConfig, wait_socket};
use crate::system_module::player::{input, move_system, transition_to_start, transition_to_ready, transition_to_gameover};
use crate::system_module::score::{update_time};
use crate::system_module::startup::{setup, set_player, set_time_score};
use crate::system_module::view::follow;
use game_ui::GameOverPlugin;

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
        .insert_resource(Scoreboard { score : 0 })
        .insert_resource(MatchboxSocket::new_ggrs("ws://127.0.0.1:3536/room"))
        .add_systems((
            // Before Match
            setup.in_schedule(OnEnter(GameState::Match)),
            wait_socket.run_if(in_state(GameState::Match)),
            transition_to_start.in_schedule(GGRSSchedule),
            transition_to_ready.in_schedule(GGRSSchedule),
            transition_to_gameover.in_schedule(GGRSSchedule),

            // Matching and Ready
            set_time_score.in_schedule(OnEnter(GameState::Ready)),
            set_player.in_schedule(OnEnter(GameState::Ready)),
            follow.run_if(in_state(GameState::Ready)),

            // Start Game
            move_system.in_schedule(GGRSSchedule).run_if(in_state(GameState::Game)),
            update_time.run_if(in_state(GameState::Game)),
            follow.run_if(in_state(GameState::Game)),

            // Game Over
        ))
        .run();
}