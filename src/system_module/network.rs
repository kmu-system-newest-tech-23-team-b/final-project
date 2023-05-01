use bevy::{prelude::*};
use bevy_ggrs::*;
use bevy_ggrs::ggrs::PlayerType;
use bevy_matchbox::MatchboxSocket;
use bevy_matchbox::prelude::*;

use crate::component::{GameState, LocalPlayer, Playerid};

pub struct GgrsConfig;

impl ggrs::Config for GgrsConfig {
    type Input = u8;
    type State = u8;
    type Address = PeerId;
}

pub fn wait_socket(mut commands: Commands, mut socket: ResMut<MatchboxSocket<SingleChannel>>, mut player_id: ResMut<Playerid>) {
    if socket.get_channel(0).is_err() { return; }
    socket.update_peers();
    let players = socket.players();
    if players.len() < 2 { return; }
    
    let mut builder = ggrs::SessionBuilder::<GgrsConfig>::new()
        .with_num_players(2)
        .with_input_delay(2);

    for (i, player) in players.into_iter().enumerate() {
        if player == PlayerType::Local { 
            player_id.id_local = socket.id().unwrap().0;
            commands.insert_resource(LocalPlayer(i)); 
        }
        else { 
            match player {
                PlayerType::Remote(peer_id) => { player_id.id_remote = peer_id.0; }
                _ => {}
            }
            
        }
        builder = builder.add_player(player, i).expect("");
    }
    let socket = socket.take_channel(0).unwrap();

    let session = builder.start_p2p_session(socket).expect("");
    commands.insert_resource(Session::P2PSession(session));
    commands.insert_resource(NextState(Some(GameState::Ready)));
}