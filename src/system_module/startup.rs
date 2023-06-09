use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::time::Stopwatch;
use bevy::tasks::TaskPool;
use bevy_ggrs::RollbackIdProvider;
use rand::prelude::StdRng;
use rand::{Rng, SeedableRng};

use crate::game_ui::styles::*;
use crate::component::{Player, PlayerSrc, GameDuration, Enemy, GamestartMenu};
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
pub fn set_player(mut commands: Commands, mut rip: ResMut<RollbackIdProvider>, player_query: Query<Entity, With<Player>>, enemy_query: Query<Entity, With<Enemy>>) {

    // player 및 emeimy 재생성
    for player_entity in player_query.iter(){ commands.entity(player_entity).despawn(); }
    for enemy_entity in enemy_query.iter() { commands.entity(enemy_entity).despawn(); }
    
    commands.spawn(
        (
            Player { handle: 0 },
            PlayerSrc { score: 0 }, 
            rip.next(), 
            SpriteBundle {
                transform: Transform::from_translation(Vec3::new(-2., 0., 100.)),
                sprite: Sprite { custom_size: Some(Vec2::new(0.3, 0.3)), color: Color::BLUE, ..default() },
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
                sprite: Sprite { custom_size: Some(Vec2::new(0.3, 0.3)),color: Color::RED, ..default() },
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

pub fn spawn_enemy(mut commands: Commands){
    let mut rng = StdRng::seed_from_u64(52);
    for i in 0..50 {
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
        let enemy_speed = rng.gen_range(0.0001..=0.3);

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

pub fn game_start_ui(mut commands: Commands, asset_server: Res<AssetServer>){
    commands.spawn(
        (
            NodeBundle {
                style: GAMESTART_MENU_STYLE,
                background_color: Color::GRAY.into(),
                ..default()
            },
            GamestartMenu {},
        )).with_children(|parent|{
            // Text
            parent.spawn(TextBundle{
                text: Text {
                    sections: vec![
                        TextSection::new(
                            "Press Space Bar",
                            get_result_text_style(&asset_server),
                        )
                    ],
                    alignment: TextAlignment::Center,
                    ..default()
                },
                ..default()
            });
        });
}

pub fn despawn_start_ui(mut commands: Commands, gamestart_menu_query: Query<Entity, With<GamestartMenu>>){
    if let Ok(gamestart_menu_entity) = gamestart_menu_query.get_single() {
        commands.entity(gamestart_menu_entity).despawn_recursive();
    }
}