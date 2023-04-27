use bevy::prelude::*;
use crate::component::{Player, PlayerSrc, GameDuration, LocalPlayer};

pub fn update_game_data(time: Res<Time>, mut gameduration: ResMut<GameDuration>, mut query_text: Query<&mut Text>,
                        query_player: Query<(&Player, &PlayerSrc)>, local_player: Option<Res<LocalPlayer>>) {
    let is_handle = match local_player {
        Some(handle) => handle.0,
        None => return
    };
    for mut text in query_text.iter_mut(){
        if text.sections[0].value.eq("게임 시간: "){
            gameduration.game_time.tick(time.delta());
            text.sections[1].value = format!("{:.1}", gameduration.game_time.elapsed_secs());
        }
        else if text.sections[0].value.eq("내 스코어: "){
            for (player, player_src) in query_player.iter(){
                if player.handle == is_handle { text.sections[1].value = player_src.score.to_string(); }
            }
        }
        else{
            for (player, player_src) in query_player.iter(){
                if player.handle != is_handle { text.sections[1].value = player_src.score.to_string(); }
            }
        }
    }
}