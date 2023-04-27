use bevy::prelude::*;
use crate::game_ui::components::*;
use crate::game_ui::styles::*;
use crate::component::{Player, PlayerSrc, LocalPlayer, GameDuration};

pub fn spawn_gameover_menu(mut commands: Commands, asset_server: Res<AssetServer>, gameduration: Res<GameDuration>,
                           query_player: Query<(&Player, &PlayerSrc)>, local_player: Option<Res<LocalPlayer>>) {
    build_gameover_menu(&mut commands, &asset_server, &gameduration, query_player, local_player);
}

pub fn despawn_gameover_menu(mut commands: Commands, gameover_menu_query: Query<Entity, With<GameoverMenu>>) {
    if let Ok(gameover_menu_entity) = gameover_menu_query.get_single() {
        commands.entity(gameover_menu_entity).despawn_recursive();
    }
}

pub fn build_gameover_menu(commands: &mut Commands, asset_server: &Res<AssetServer>, gameduration: &Res<GameDuration>,
                           query_player: Query<(&Player, &PlayerSrc)>, local_player: Option<Res<LocalPlayer>>) -> Entity {
    let mut player1_score = String::new();
    let mut player2_score = String::new();
    let is_handle = match local_player {
        Some(handle) => handle.0,
        None => 2
    };
    for (player, player_src) in query_player.iter(){
        if player.handle == is_handle { player1_score = player_src.score.to_string(); }
        else { player2_score = player_src.score.to_string();}
    }

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
                                    format!("사용자 1"),
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
                                    format!("    스코어: {}", player1_score),
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
                                    format!("사용자 2"),
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
                                    format!("    스코어: {}", player2_score),
                                    get_result_text_style(&asset_server),
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