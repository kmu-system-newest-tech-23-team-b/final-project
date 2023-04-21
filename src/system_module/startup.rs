use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::tasks::TaskPool;
use bevy_ggrs::RollbackIdProvider;

use crate::component::Player;
use crate::system_module::view::MAP_SIZE;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scaling_mode = ScalingMode::FixedVertical(10.);
    commands.spawn(camera_bundle);
    for i in 0..=MAP_SIZE {
        commands.spawn(SpriteBundle {
            transform: Transform::from_translation(Vec3::new(
                0., i as f32 - MAP_SIZE as f32 / 2., 0.)),
            sprite: Sprite {
                color: Color::ANTIQUE_WHITE,
                custom_size: Some(Vec2::new(MAP_SIZE as f32, 0.05)),
                ..default()
            },
            ..default()
        });
    }

    commands.spawn(
        TextBundle::from_sections([
            TextSection::new(
                "Score: ",
                TextStyle {
                    font: asset_server.load("NotoSansKR-Bold.otf"),
                    font_size: 50.0,
                    color: Color::BLACK,
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load("NotoSansKR-Bold.otf"),
                font_size: 50.0,
                color: Color::BLACK,
            }),
        ])
            .with_style(Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Px(10.0),
                    left: Val::Px(10.0),
                    ..default()
                },
                ..default()
            }),
    );
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

pub fn play_music(_: Commands, asset_server: Res<AssetServer>, audio: Res<Audio>) {
    let pool = TaskPool::new();
    pool.scope(|s| {
        s.spawn(async {
            audio.play_with_settings(
                asset_server.load("ganggangsullae.mp3"),
                PlaybackSettings::LOOP.with_volume(0.1));
        });
    });
}