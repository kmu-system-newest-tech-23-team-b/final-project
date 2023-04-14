use bevy::{prelude::*};
use bevy_ggrs::*;
use bevy_matchbox::MatchboxSocket;
use bevy_matchbox::prelude::*;

pub struct GgrsConfig;

impl ggrs::Config for GgrsConfig {
    type Input = u8;
    type State = u8;
    type Address = PeerId;
}

pub fn wait_socket(mut commands: Commands, mut socket: ResMut<MatchboxSocket<SingleChannel>>) {
    if socket.get_channel(0).is_err() { return; }
    socket.update_peers();
    let players = socket.players();
    if players.len() < 2 { return; }
    let mut builder = ggrs::SessionBuilder::<GgrsConfig>::new()
        .with_num_players(2)
        .with_input_delay(2);
    for (i, player) in players.into_iter().enumerate() {
        builder = builder.add_player(player, i).expect("");
    }
    let socket = socket.take_channel(0).unwrap();
    let session = builder.start_p2p_session(socket).expect("");
    commands.insert_resource(bevy_ggrs::Session::P2PSession(session));
}