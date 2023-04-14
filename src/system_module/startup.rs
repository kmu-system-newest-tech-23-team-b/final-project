use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy_ggrs::RollbackIdProvider;

use crate::component::Player;

pub fn setup(mut commands: Commands) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scaling_mode = ScalingMode::FixedVertical(10.);
    commands.spawn(camera_bundle);
}

pub fn spawn(mut commands: Commands, mut rip: ResMut<RollbackIdProvider>) {
    commands.spawn((Player { handle: 0 }, rip.next(), SpriteBundle {
        transform: Transform::from_translation(Vec3::new(-1., -1., 0.)),
        sprite: Sprite { color: Color::BLUE, ..default() },
        ..default()
    }));
    commands.spawn((Player { handle: 1 }, rip.next(), SpriteBundle {
        transform: Transform::from_translation(Vec3::new(1., 1., 0.)),
        sprite: Sprite { color: Color::RED, ..default() },
        ..default()
    }));
}