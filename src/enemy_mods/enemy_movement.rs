use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::components::*;

pub fn enemy_movement(
    mut position: Query<&mut Transform, With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
    ){
    let window = window_query.get_single().unwrap();
    for mut transform in &mut position {
        transform.translation.y -= 300.0 * time.delta_seconds();
    }

}
