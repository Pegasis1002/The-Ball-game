use bevy::prelude::*;
use crate::enemy_mods::spawn_enemy::*;
use crate::enemy_mods::enemy_movement::*;

pub struct EnemyPlugin;
#[derive(Component)]
pub struct Enemy;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_systems(Update, spawn_enemy)
            .add_systems(Update, enemy_movement);
    }
}
