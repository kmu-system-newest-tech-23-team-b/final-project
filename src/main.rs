use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

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

fn setup(mut commands: Commands) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scaling_mode = ScalingMode::FixedVertical(10.);
    commands.spawn(camera_bundle);
}

fn spawn(mut commands: Commands) {
    commands.spawn((
        Player,
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

#[derive(Component)]
struct Player;

fn move_system(keys: Res<Input<KeyCode>>, mut query: Query<&mut Transform, With<Player>>) {
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