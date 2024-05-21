use bevy::prelude::*;

pub const PLAYER_SIZE:f32 = 0.8;
pub const ENEMY_SIZE:f32 = 0.8;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Position;

#[derive(Component)]
pub struct Size;

#[derive(Component)]
pub struct Score{
    value: u64,
}
