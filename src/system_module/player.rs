use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy_ggrs::{ggrs, PlayerInputs};

use crate::component::{Player, GameState, Scoreboard};
use crate::system_module::network::GgrsConfig;
use crate::system_module::view::MAP_SIZE;

const UP: u8 = 1 << 0;
const DOWN: u8 = 1 << 1;
const LEFT: u8 = 1 << 2;
const RIGHT: u8 = 1 << 3;
const SPACE: u8 = 1 << 4; // game start
const ENTER: u8 = 1 << 5; // game over -> game ready
const ESC : u8 = 1 << 6; // game over 게임이 끝나는 이벤트가 아직 개발 중이기에 ESC를 눌렀을 때 게임이 끝난 걸로 테스트 진행함.
                               // 어차피 GameState만 GameOver로 변경하면 되는 걸로 진행함

pub fn input(_: In<ggrs::PlayerHandle>, keys: Res<Input<KeyCode>>) -> u8 {
    let mut input = 0;
    if keys.any_pressed([KeyCode::Up]) { input |= UP; }
    if keys.any_pressed([KeyCode::Down]) { input |= DOWN; }
    if keys.any_pressed([KeyCode::Left]) { input |= LEFT }
    if keys.any_pressed([KeyCode::Right]) { input |= RIGHT; }
    if keys.any_pressed([KeyCode::Space]) { input |= SPACE; }
    if keys.any_pressed([KeyCode::Escape]) { input |= ESC; }
    if keys.any_pressed([KeyCode::Return]) { input |= ENTER; }
    input
}

// Match 후 Ready 상태에서 두 플레이어 중 아무나 SpaceBar를 누르면 Game 상태로 돌입하는 함수
pub fn transition_to_start(pi: Res<PlayerInputs<GgrsConfig>>, query_player: Query<&Player>, 
                        game_state: Res<State<GameState>>, mut next_game_state: ResMut<NextState<GameState>>) {
    for player in query_player.iter() {
        let (input, _) = pi[player.handle];
        if input & SPACE != 0 { // Ready to Game
            if game_state.0 != GameState::Game{
                next_game_state.set(GameState::Game);
            }
        }
    }
}

// Game Over 이후 다시 시작하기 위한 기능으로 Enter 누를 시 Ready 상태로 돌아감
pub fn transition_to_ready(pi: Res<PlayerInputs<GgrsConfig>>, query_player: Query<&Player>, 
                        game_state: Res<State<GameState>>, mut commands: Commands) {
    for player in query_player.iter(){
        let (input, _) = pi[player.handle];
        if input & ENTER != 0 {
            if game_state.0 != GameState::Ready {
                commands.insert_resource(NextState(Some(GameState::Ready)));
            }
        } 
    }
}

// 현재 Game Over에 대한 이벤트가 없어서 임시로 ECS 누를 시 GameOver되게 하는 함수
pub fn transition_to_gameover(pi: Res<PlayerInputs<GgrsConfig>>, query_player: Query<&Player>, 
                        game_state: Res<State<GameState>>, mut commands: Commands) {
    for player in query_player.iter() {
        let (input, _) = pi[player.handle];
        if input & ESC != 0 {
            if game_state.0 != GameState::GameOver {
                commands.insert_resource(NextState(Some(GameState::GameOver)));
            }
        }
    }  
}

pub fn move_system(pi: Res<PlayerInputs<GgrsConfig>>, mut query: Query<(&mut Transform, &Player)>, mut scoreboard: ResMut<Scoreboard>) {
    for (mut transform, player) in query.iter_mut() {
        let (input, _) = pi[player.handle];
        let mut direction = Vec2::ZERO;
        if input & UP != 0 {
            direction.y += 0.1;
            scoreboard.score += 1;
        }
        if input & DOWN != 0 {
            direction.y -= 0.1;
            scoreboard.score += 1;
        }
        if input & RIGHT != 0 {
            direction.x += 0.1;
            scoreboard.score += 1;
        }
        if input & LEFT != 0 {
            direction.x -= 0.1;
            scoreboard.score += 1;
        }
        if direction == Vec2::ZERO { continue; }
        let limit = Vec2::splat(MAP_SIZE as f32 / 2. - 0.5);
        let pos = (transform.translation.xy() + direction).clamp(-limit, limit);
        transform.translation.x = pos.x;
        transform.translation.y = pos.y;
    }
}