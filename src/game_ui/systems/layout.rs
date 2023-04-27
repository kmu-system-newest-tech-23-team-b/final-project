use bevy::prelude::*;
use crate::game_ui::components::*;
use crate::game_ui::styles::*;
use crate::component::{GameDuration, Scoreboard};

pub fn spawn_gameover_menu(mut commands: Commands, asset_server: Res<AssetServer>, gameduration: Res<GameDuration>, scoreboard: Res<Scoreboard>) {
    build_gameover_menu(&mut commands, &asset_server, &gameduration, &scoreboard);
}

pub fn despawn_gameover_menu(mut commands: Commands, gameover_menu_query: Query<Entity, With<Gameover_Menu>>) {
    if let Ok(gameover_menu_entity) = gameover_menu_query.get_single() {
        commands.entity(gameover_menu_entity).despawn_recursive();
    }
}

pub fn build_gameover_menu(commands: &mut Commands, asset_server: &Res<AssetServer>, gameduration: &Res<GameDuration>, scoreboard: &Res<Scoreboard>) -> Entity {
    let gameover_menu_entity = commands
        .spawn(
            (
            NodeBundle {
                style: GAMEOVER_MENU_STYLE,
                background_color: Color::GRAY.into(),
                ..default()
            },
            Gameover_Menu {},
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
                                    get_user_text_style(&asset_server),
                                )
                            ],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                });
            // 결과 출력
            parent.spawn(
                NodeBundle{
                    style: RESULT_STYLE,
                    ..default()
                }).with_children(|parent|{
                    // 게임 시간 Text
                    parent.spawn(TextBundle{
                        text: Text {
                            sections: vec![
                                TextSection::new(
                                    format!("게임 시간: {:.0}", gameduration.game_time.elapsed_secs()),
                                    get_result_text_style(&asset_server),
                                )
                            ],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                    // 게임 스코어 Text
                    parent.spawn(TextBundle{
                        text: Text {
                            sections: vec![
                                TextSection::new(
                                    format!("    스코어: {}", scoreboard.score.to_string()),
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
                                    get_user_text_style(&asset_server),
                                )
                            ],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                });
            // 결과 출력
            parent.spawn(
                NodeBundle{
                    style: RESULT_STYLE,
                    ..default()
                }).with_children(|parent|{
                    // 게임 시간 Text
                    parent.spawn(TextBundle{
                        text: Text {
                            sections: vec![
                                TextSection::new(
                                    format!("게임 시간: {:.0}", gameduration.game_time.elapsed_secs()),
                                    get_result_text_style(&asset_server),
                                )
                            ],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                    // 게임 스코어 Text
                    parent.spawn(TextBundle{
                        text: Text {
                            sections: vec![
                                TextSection::new(
                                    format!("    스코어: {}", scoreboard.score.to_string()),
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
                    style: Button_STYLE,
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
                    style: Button_STYLE,
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