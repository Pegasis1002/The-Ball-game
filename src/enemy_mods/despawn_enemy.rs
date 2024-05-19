use bevy::prelude::*;
use crate::components::*;

pub fn despawn_enemy(
    mut commands: Commands,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
    ){
    for (entity, transform) in enemy_query.iter() {
        if transform.translation.y < 0.0 {
            commands.entity(entity).despawn();
        }
    }
}
