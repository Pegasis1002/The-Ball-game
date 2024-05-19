use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::components::Player;

pub const PLAYER_SPEED: f32 = 500.0;

pub fn player_movement(
    time: Res<Time>,
    mut position: Query<&mut Transform, With<Player>>,
    keys: Res<ButtonInput<KeyCode>>,
    window_query: Query<&Window, With<PrimaryWindow>>){
    let width = window_query.get_single().unwrap().width();
    let mut position = position.single_mut();

    //player_movement
    if keys.pressed (KeyCode::ArrowLeft) {
        position.translation.x -= PLAYER_SPEED * time.delta_seconds();
    }
    if keys.pressed (KeyCode::ArrowRight){
        position.translation.x += PLAYER_SPEED * time.delta_seconds();
    }

    //Boundaries/Collisions
    if position.translation.x > width - 90.0{
        position.translation.x = width - 90.0;
    }
    if position.translation.x < 90.0{
        position.translation.x = 90.0;
    }
}
