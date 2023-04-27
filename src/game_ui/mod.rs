pub mod components;
mod styles;
mod systems;

use bevy::prelude::*;
use systems::layout::*;
use crate::component::{GameState};
use self::systems::interactions::{interact_with_replay_button, interact_with_quit_button};

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app
            // OnEnter State Systems
            .add_system(spawn_gameover_menu.in_schedule(OnEnter(GameState::GameOver)))
            //Systems
            .add_systems(
                (
                    interact_with_replay_button,
                    interact_with_quit_button,
                ).in_set(OnUpdate(GameState::GameOver))
            )
            // OnExit State Systems
            .add_system(despawn_gameover_menu.in_schedule(OnExit(GameState::GameOver)));
    }
}