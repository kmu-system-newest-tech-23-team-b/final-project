use bevy::{prelude::*};
use bevy_ggrs::*;
use bevy_matchbox::prelude::*;

use crate::system_module::network::{GgrsConfig, wait_socket};
use crate::system_module::player::{input, move_system};
use crate::system_module::startup::{setup, spawn};

mod system_module;
mod component;

fn main() {
    let mut app = App::new();
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
        .insert_resource(MatchboxSocket::new_ggrs("ws://127.0.0.1:3536/room"))
        .add_startup_systems((setup, spawn))
        .add_systems((move_system.in_schedule(GGRSSchedule), wait_socket)).run();
}