use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                fit_canvas_to_parent: true,
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::WHITE))
        .add_startup_systems((setup, spawn))
        .run();
}

fn setup(mut commands: Commands) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scaling_mode = ScalingMode::FixedVertical(10.);
    commands.spawn(camera_bundle);
}

fn spawn(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::BLUE,
                custom_size: Some(Vec2::new(1., 1.)),
                ..default()
            },
            ..default()
        },
    ));
}