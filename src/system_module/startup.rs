use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::time::Stopwatch;
use bevy::tasks::TaskPool;
use bevy_ggrs::RollbackIdProvider;

use crate::component::{Player, PlayerSrc, GameDuration};
use crate::system_module::view::MAP_SIZE;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>, player_query: Query<Entity, With<Player>>) {

    // 한명이 나가서 Match 상태로 돌아오면 일단 모든 player는 사라져야 함
    for entity in player_query.iter(){  commands.entity(entity).despawn(); }


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
                "게임 시간: ",
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
    commands.spawn(
        TextBundle::from_sections([
            TextSection::new(
                "내 스코어: ",
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
                    top: Val::Px(50.0),
                    left: Val::Px(10.0),
                    ..default()
                },
                ..default()
            }),
    );
    commands.spawn(
        TextBundle::from_sections([
            TextSection::new(
                "동료 스코어: ",
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
                    top: Val::Px(90.0),
                    left: Val::Px(10.0),
                    ..default()
                },
                ..default()
            }),
    );
}

// 게임 시작 시, 게임 종료 후 다시 시작 시 시간과 score를 set 해주는 함수
pub fn set_time_score(mut gameduration: ResMut<GameDuration>, mut query: Query<&mut Text>){
    for (i, mut text) in query.iter_mut().enumerate(){
        if i == 0{
            gameduration.game_time = Stopwatch::new();
            text.sections[1].value = format!("{:.1}", gameduration.game_time.elapsed_secs());
        }else{
            text.sections[1].value = 0.to_string();
        }
    }
}

// 게임 시작 시, 게임 종료 후 다시 시작 시 player를 set 해주는 함수
pub fn set_player(mut commands: Commands, mut rip: ResMut<RollbackIdProvider>, player_query: Query<Entity, With<Player>>) {

    // player 및 emeimy 재생성
    for entity in player_query.iter(){  commands.entity(entity).despawn(); }
    
    commands.spawn(
        (
            Player { handle: 0 },
            PlayerSrc { score: 0 }, 
            rip.next(), 
            SpriteBundle {
                transform: Transform::from_translation(Vec3::new(-2., 0., 100.)),
                sprite: Sprite { color: Color::BLUE, ..default() },
                ..default()
            },
        )
    );
    commands.spawn(
        (
            Player { handle: 1},
            PlayerSrc { score: 0 }, 
            rip.next(), 
            SpriteBundle {
                transform: Transform::from_translation(Vec3::new(2., 0., 100.)),
                sprite: Sprite { color: Color::RED, ..default() },
                ..default()
            },
        )
    );
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