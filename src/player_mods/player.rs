use bevy::prelude::*;
use crate::player_mods::sp_player::*;
use crate::player_mods::player_movement::*;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, player_movement);
    }
}
