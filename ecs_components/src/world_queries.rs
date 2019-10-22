use crate::IsPlayer;
use shared_types::PlayingGameState;
use specs::{Read, ReadStorage, World};

pub fn get_playing_state(world: &World) -> PlayingGameState {
    let state: (Read<'_, PlayingGameState>) = world.system_data();
    *state
}

pub fn is_player_dead(world: &World) -> bool {
    let is_player: ReadStorage<IsPlayer> = world.system_data();
    is_player.is_empty()
}
