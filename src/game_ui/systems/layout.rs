use bevy::prelude::*;
use std::fs::File;
use std::io::Write;
use chrono::{Local, Datelike, Timelike};
use crate::game_ui::components::*;
use crate::game_ui::styles::*;
use crate::component::{Player, PlayerSrc, LocalPlayer, GameDuration, Playerid};

pub fn spawn_gameover_menu(mut commands: Commands, asset_server: Res<AssetServer>, gameduration: Res<GameDuration>,
                           query_player: Query<(&Player, &PlayerSrc)>, local_player: Option<Res<LocalPlayer>>, player_id: Res<Playerid>) {
    build_gameover_menu(&mut commands, &asset_server, &gameduration, query_player, local_player, &player_id);
}

pub fn despawn_gameover_menu(mut commands: Commands, gameover_menu_query: Query<Entity, With<GameoverMenu>>) {
    if let Ok(gameover_menu_entity) = gameover_menu_query.get_single() {
        commands.entity(gameover_menu_entity).despawn_recursive();
    }
}

pub fn build_gameover_menu(commands: &mut Commands, asset_server: &Res<AssetServer>, gameduration: &Res<GameDuration>,
                           query_player: Query<(&Player, &PlayerSrc)>, local_player: Option<Res<LocalPlayer>>,
                           player_id: &Res<Playerid>) -> Entity {
    let mut player1_score = String::new();
    let player1_id = player_id.id_0.to_string();
    let mut player2_score = String::new();
    let player2_id = player_id.id_1.to_string();
    let mut final_scroe = 0;
    let now = Local::now();
    let is_handle = match local_player {
        Some(handle) => handle.0,
        None => 2
    };
    for (player, player_src) in query_player.iter(){
        final_scroe += player_src.score;
        if player.handle == is_handle { player1_score = player_src.score.to_string(); }
        else { player2_score = player_src.score.to_string();}
    }
    let write_result = format!("[{}-{}-{} {:02}:{:02}:{:02}에 진행한 게임]\n\n게임 시간: {:.0}초\n내 점수: {}점\n동료 점수: {}점\n\n최종 점수: {}점\n\n", 
                                        now.year(), now.month(), now.day(), now.hour(), now.minute(), now.second(), 
                                        gameduration.game_time.elapsed_secs(),
                                        player1_score,
                                        player2_score,
                                        final_scroe.to_string());
    

    let mut data_file = File::create("result.txt").expect("creation failed");
    data_file.write(write_result.as_bytes()).expect("write failed");

    let gameover_menu_entity = commands
        .spawn(
            (
            NodeBundle {
                style: GAMEOVER_MENU_STYLE,
                background_color: Color::GRAY.into(),
                ..default()
            },
            GameoverMenu {},
        ))

        .with_children(|parent|{
            // == Title ==
            parent.spawn(
                NodeBundle{
                    style: TITLE_STYLE,
                    ..default()
                }).with_children(|parent|{
                    // Imgae 1
                    parent.spawn(ImageBundle{
                        style: IMAGE_STYLE,
                        image: asset_server.load("ball_blue_large.png").into(),
                        ..default()
                    });
                    // Text
                    parent.spawn(TextBundle{
                        text: Text {
                            sections: vec![
                                TextSection::new(
                                    "Game Over!!",
                                    get_title_text_style(&asset_server),
                                )
                            ],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                    // Image 2
                    parent.spawn(ImageBundle{
                        style: IMAGE_STYLE,
                        image: asset_server.load("ball_blue_large.png").into(),
                        ..default()
                    });
                });
            // == Game Result ==
            // 시간 출력
            parent.spawn(
                NodeBundle{
                    style: USER_STYLE,
                    ..default()
                }).with_children(|parent|{
                    parent.spawn(TextBundle{
                        text: Text {
                            sections: vec![
                                TextSection::new(
                                    format!("게임 시간: {:.0}초", gameduration.game_time.elapsed_secs()),
                                    get_user_text_style(&asset_server),
                                )
                            ],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                });
            // 사용자 출력
            parent.spawn(
                NodeBundle{
                    style: USER_STYLE,
                    ..default()
                }).with_children(|parent|{
                    parent.spawn(TextBundle{
                        text: Text {
                            sections: vec![
                                TextSection::new(
                                    format!("나 (socket id: {})", player1_id),
                                    get_result_text_style(&asset_server),
                                )
                            ],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                    parent.spawn(TextBundle{
                        text: Text {
                            sections: vec![
                                TextSection::new(
                                    format!("    스코어: {}점", player1_score),
                                    get_result_text_style(&asset_server),
                                )
                            ],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                });
            
            // 사용자 출력
            parent.spawn(
                NodeBundle{
                    style: USER_STYLE,
                    ..default()
                }).with_children(|parent|{
                    parent.spawn(TextBundle{
                        text: Text {
                            sections: vec![
                                TextSection::new(
                                    format!("동료 (socket id: {})", player2_id),
                                    get_result_text_style(&asset_server),
                                )
                            ],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                    parent.spawn(TextBundle{
                        text: Text {
                            sections: vec![
                                TextSection::new(
                                    format!("    스코어: {}점", player2_score),
                                    get_result_text_style(&asset_server),
                                )
                            ],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                });

            // 최종 스코어 출력
            parent.spawn(
                NodeBundle{
                    style: USER_STYLE,
                    ..default()
                }).with_children(|parent|{
                    parent.spawn(TextBundle{
                        text: Text {
                            sections: vec![
                                TextSection::new(
                                    format!("최종 점수: {}점", final_scroe.to_string()),
                                    get_user_text_style(&asset_server),
                                )
                            ],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                });

            // == Replay Button ==
            parent.spawn((
                ButtonBundle {
                    style: BUTTON_STYLE,
                    background_color: NORMAL_BUTTON_COLOR.into(),
                    ..default()
                },
                ReplayButton {}
            ))
            .with_children(|parent|{
                parent.spawn(TextBundle{
                    text: Text{
                        sections: vec![TextSection::new(
                            "RePlay",
                            get_button_text_style(&asset_server),
                        )],
                        alignment: TextAlignment::Center,
                        ..default()
                    },
                    ..default()
                });
            });
            // == Quit Button ==
            parent.spawn((
                ButtonBundle {
                    style: BUTTON_STYLE,
                    background_color: NORMAL_BUTTON_COLOR.into(),
                    ..default()
                },
                QuitButton {}
            ))
            .with_children(|parent|{
                parent.spawn(TextBundle{
                    text: Text{
                        sections: vec![TextSection::new(
                            "Quit",
                            get_button_text_style(&asset_server),
                        )],
                        alignment: TextAlignment::Center,
                        ..default()
                    },
                    ..default()
                });
            });
        })
        .id();
    gameover_menu_entity
}