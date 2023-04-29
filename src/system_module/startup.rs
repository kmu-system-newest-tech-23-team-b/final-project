use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::tasks::TaskPool;
use bevy_ggrs::RollbackIdProvider;
use rand::Rng;


use crate::component::{Player, Enemy};
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

pub fn spawn_enemy(mut commands: Commands){
    let mut rng = rand::thread_rng();
    for i in 0..10 {
        let ran_num = rng.gen_range(0..4);  // 0 : 왼쪽 벽, 1 : 오른쪽 벽, 2 : 위쪽 벽, 3 : 아랫쪽 벽
        let map_size = MAP_SIZE as f32 + 0.3;
        let pos_x = match ran_num {
            0 => { -map_size / 2.0 }
            1 => { map_size / 2.0 }
            2 => { rng.gen_range(-map_size / 2.0..map_size / 2.0) }
            3 => { rng.gen_range(-map_size / 2.0..map_size / 2.0) }
            _ => unreachable!(),
        };
        let pos_y = match ran_num {
            0 => { rng.gen_range(-map_size / 2.0..map_size / 2.0) }
            1 => { rng.gen_range(-map_size / 2.0..map_size / 2.0) }
            2 => { map_size / 2.0 }
            3 => { -map_size / 2.0 }
            _ => unreachable!(),
        };
        let enemy_speed = rng.gen_range(1.5..=2.2);

        commands.spawn((
            Enemy { handle: i, position: Vec2::new(pos_x, pos_y).normalize(), speed: enemy_speed },
            SpriteBundle {
                transform: Transform::from_xyz(pos_x, pos_y, 100.),
                sprite: Sprite { custom_size: Some(Vec2::new(0.3, 0.3)), color: Color::BLACK, ..default() },
                ..default()
            }));
    }
}

pub fn despawn_enemy(mut commands: Commands, mut query: Query<(Entity, &Transform), With<Enemy>>,){
    let map_size = MAP_SIZE as f32 + 0.3;

    for (enemy_entity, enemy_transform) in query.iter_mut(){
        if enemy_transform.translation.x < -map_size / 2.0 || enemy_transform.translation.x > map_size / 2.0 ||
            enemy_transform.translation.y < -map_size / 2.0 || enemy_transform.translation.y > map_size / 2.0 {
            commands.entity(enemy_entity).despawn();
        }
    }
}