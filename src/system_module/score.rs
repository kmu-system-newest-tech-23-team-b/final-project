use bevy::prelude::*;

use crate::component::Scoreboard;

pub fn update_score(scoreboard: Res<Scoreboard>, mut query: Query<&mut Text>) {
    let mut text = query.single_mut();
    text.sections[1].value = scoreboard.score.to_string();
}

// fn score_timer(time: Res<Time>, mut scoreboard: ResMut<Scoreboard>){
//     scoreboard.score += time.delta_seconds() as usize;
// }