use bevy::prelude::*;

use system_module::player::move_system;
use system_module::startup::{setup, spawn};

mod system_module;
mod component;

fn main() {
    App::new()
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
        .add_startup_systems((setup, spawn))
        .add_system(move_system)
        .run();
}