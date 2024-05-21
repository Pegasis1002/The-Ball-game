use bevy::prelude::*;
use crate::GameState;
use crate::env_mods::edge_blocks::edge_blocks;

pub struct EnvPlugin;
impl Plugin for EnvPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (edge_blocks).run_if(in_state(GameState::Playing)));
    }
}
