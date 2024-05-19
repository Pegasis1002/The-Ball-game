use bevy::prelude::*;
use crate::enemy_mods::spawn_enemy::*;
use crate::enemy_mods::enemy_movement::*;
use crate::enemy_mods::despawn_enemy::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
            .add_systems(Update, (spawn_enemy, despawn_enemy))
            .add_systems(Update, enemy_movement);
    }
}
