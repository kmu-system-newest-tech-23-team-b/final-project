use bevy::app::AppExit;
use bevy::prelude::*;

use crate::component::GameState;
use crate::game_ui::components::*;
use crate::game_ui::styles::{HOVERED_BUTTON_COLOR, NORMAL_BUTTON_COLOR, PRESSED_BUTTON_COLOR};

pub fn interact_with_replay_button(mut button_query: Query<(&Interaction, &mut BackgroundColor),(Changed<Interaction>, With<ReplayButton>)>,
                                   mut game_state_next_state: ResMut<NextState<GameState>>) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut(){
        match *interaction {
            Interaction::Clicked => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                game_state_next_state.set(GameState::Ready);
            } 
            Interaction::Hovered => {
                *background_color = HOVERED_BUTTON_COLOR.into();
            }
            Interaction::None => {
                *background_color = NORMAL_BUTTON_COLOR.into();
            }
        }
    }
}

pub fn interact_with_quit_button(mut button_query: Query<(&Interaction, &mut BackgroundColor),(Changed<Interaction>, With<QuitButton>)>,
                                 mut game_exit_event_writer: EventWriter<AppExit>) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut(){
        match *interaction {
            Interaction::Clicked => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                game_exit_event_writer.send(AppExit);
            } 
            Interaction::Hovered => {
                *background_color = HOVERED_BUTTON_COLOR.into();
            }
            Interaction::None => {
                *background_color = NORMAL_BUTTON_COLOR.into();
            }
        }
    }
}