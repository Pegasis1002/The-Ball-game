use bevy::prelude::*;
use crate::player_mods::player::*;


pub const PLAYER_SPEED: f32 = 500.0;
pub fn player_movement(
    time: Res<Time>,
    mut position: Query<&mut Transform, With<Player>>,
    keys: Res<ButtonInput<KeyCode>>,
    ){
    let mut position = position.single_mut();
    if keys.pressed (KeyCode::ArrowLeft) {
        position.translation.x -= PLAYER_SPEED * time.delta_seconds();
    }
    if keys.pressed (KeyCode::ArrowRight){
        position.translation.x += PLAYER_SPEED * time.delta_seconds();
    }
}
