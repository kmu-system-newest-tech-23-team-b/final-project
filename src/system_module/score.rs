use bevy::prelude::*;
use crate::component::{GameDuration, Scoreboard};

pub fn update_time(time: Res<Time>, mut gameduration: ResMut<GameDuration>, mut query: Query<&mut Text>, scoreboard: Res<Scoreboard>) {
    for mut text in query.iter_mut(){
        if text.sections[0].value.eq("게임 시간: "){
            gameduration.game_time.tick(time.delta());
            text.sections[1].value = format!("{:.1}", gameduration.game_time.elapsed_secs());
        }else{
            text.sections[1].value = scoreboard.score.to_string();
        }
    }
}