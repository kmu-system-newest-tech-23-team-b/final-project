use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy::app::AppExit;
use bevy_ggrs::{ggrs, PlayerInputs};

use crate::component::{Player, GameState, PlayerSrc};
use crate::system_module::network::GgrsConfig;
use crate::system_module::view::MAP_SIZE;
use crate::game_ui::components::{ReplayButton, QuitButton};

const UP: u8 = 1 << 0;
const DOWN: u8 = 1 << 1;
const LEFT: u8 = 1 << 2;
const RIGHT: u8 = 1 << 3;
const SPACE: u8 = 1 << 4; // game start
const ENTER: u8 = 1 << 5; // game over -> game ready
const CLICK_REPLAY: u8 = 1 << 5;
const ESC : u8 = 1 << 6; // game over 게임이 끝나는 이벤트가 아직 개발 중이기에 ESC를 눌렀을 때 게임이 끝난 걸로 테스트 진행함.
                               // 어차피 GameState만 GameOver로 변경하면 되는 걸로 진행함
const QUIT: u8 = 1 << 7;
const CLICK_QUIT: u8 = 1 << 7; // game out

pub fn input(_: In<ggrs::PlayerHandle>, keys: Res<Input<KeyCode>>, mouses: Res<Input<MouseButton>>,
             replay_button_query: Query<&Interaction,(Changed<Interaction>, With<ReplayButton>)>,
             quit_button_query: Query<&Interaction,(Changed<Interaction>, With<QuitButton>)>
            ) -> u8 {
    let mut input = 0;
    if keys.any_pressed([KeyCode::Up]) { input |= UP; }
    if keys.any_pressed([KeyCode::Down]) { input |= DOWN; }
    if keys.any_pressed([KeyCode::Left]) { input |= LEFT }
    if keys.any_pressed([KeyCode::Right]) { input |= RIGHT; }
    if keys.any_pressed([KeyCode::Space]) { input |= SPACE; }
    if keys.any_pressed([KeyCode::Escape]) { 
        input |= ESC; 
        
    }
    if keys.any_pressed([KeyCode::Return]) { input |= ENTER; }

    if mouses.any_pressed([MouseButton::Left]) {
        if let Ok(interaction) = replay_button_query.get_single(){
            match *interaction {
                Interaction::Clicked => {
                    input |= CLICK_REPLAY; 
                } 
                Interaction::Hovered => {}
                Interaction::None => {}
            }
        }
        if let Ok(interaction) = quit_button_query.get_single(){
            match *interaction {
                Interaction::Clicked => {
                    input |= CLICK_QUIT; 
                } 
                Interaction::Hovered => {}
                Interaction::None => {}
            }
        }
    }
    input
}

// Match 후 Ready 상태에서 두 플레이어 중 아무나 SpaceBar를 누르면 Game 상태로 돌입하는 함수
pub fn transition_state(pi: Res<PlayerInputs<GgrsConfig>>, query_player: Query<&Player>, 
                        game_state: Res<State<GameState>>, mut next_game_state: ResMut<NextState<GameState>>, mut commands: Commands,
                        mut game_exit_event_writer: EventWriter<AppExit>) {
    for player in query_player.iter() {
        let (input, _) = pi[player.handle];
        if input & SPACE != 0 { // Ready to Game
            if game_state.0 != GameState::Game{
                next_game_state.set(GameState::Game);
            }
        }
        // Game Over 이후 다시 시작하기 위한 기능으로 Enter 누를 시 Ready 상태로 돌아감
        else if input & ENTER != 0 {
            if game_state.0 != GameState::Ready {
                commands.insert_resource(NextState(Some(GameState::Ready)));
            }
        } 
        // 현재 Game Over에 대한 이벤트가 없어서 임시로 ECS 누를 시 GameOver되게 하는 함수
        else if input & ESC != 0 {
            if game_state.0 != GameState::GameOver {
                // 게임 종료 이벤트 발생하면 스코어 싱크 맞추는 코드 추가하기
                commands.insert_resource(NextState(Some(GameState::GameOver)));
            }
        }
        // Game Over에서 Quit으로 변경하는 함수
        else if input & QUIT != 0 {
            game_exit_event_writer.send(AppExit); // 일단 한명 나가면 둘다 나가는 걸로 하자.
            // commands.insert_resource(NextState(Some(GameState::Match)));
        }
    }
}

pub fn move_system(pi: Res<PlayerInputs<GgrsConfig>>, mut query: Query<(&mut Transform, &Player, &mut PlayerSrc)>) {
    for (mut transform, player, mut player_src) in query.iter_mut() {
        let (input, _) = pi[player.handle];
        let mut direction = Vec2::ZERO;
        if input & UP != 0 {
            direction.y += 0.1;
            player_src.score += 1;
        }
        if input & DOWN != 0 {
            direction.y -= 0.1;
            player_src.score += 1;
        }
        if input & RIGHT != 0 {
            direction.x += 0.1;
            player_src.score += 1;
        }
        if input & LEFT != 0 {
            direction.x -= 0.1;
            player_src.score += 1;
        }
        if direction == Vec2::ZERO { continue; }
        let limit = Vec2::splat(MAP_SIZE as f32 / 2. - 0.5);
        let pos = (transform.translation.xy() + direction).clamp(-limit, limit);
        transform.translation.x = pos.x;
        transform.translation.y = pos.y;
    }
}