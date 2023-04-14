use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy_ggrs::RollbackIdProvider;

use crate::component::Player;
use crate::system_module::view::{GRID_WIDTH, MAP_SIZE};

pub fn setup(mut commands: Commands) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scaling_mode = ScalingMode::FixedVertical(10.);
    commands.spawn(camera_bundle);
    for i in 0..=MAP_SIZE {
        commands.spawn(SpriteBundle {
            transform: Transform::from_translation(Vec3::new(
                0., i as f32 - MAP_SIZE as f32 / 2., 0.)),
            sprite: Sprite {
                color: Color::ANTIQUE_WHITE,
                custom_size: Some(Vec2::new(MAP_SIZE as f32, GRID_WIDTH)),
                ..default()
            },
            ..default()
        });
    }
}

pub fn spawn(mut commands: Commands, mut rip: ResMut<RollbackIdProvider>) {
    commands.spawn((Player { handle: 0 }, rip.next(), SpriteBundle {
        transform: Transform::from_translation(Vec3::new(-2., 0., 100.)),
        sprite: Sprite { color: Color::BLUE, ..default() },
        ..default()
    }));
    commands.spawn((Player { handle: 1 }, rip.next(), SpriteBundle {
        transform: Transform::from_translation(Vec3::new(2., 0., 100.)),
        sprite: Sprite { color: Color::RED, ..default() },
        ..default()
    }));
}