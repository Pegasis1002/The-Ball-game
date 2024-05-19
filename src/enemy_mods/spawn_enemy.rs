use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::Rng;
use crate::components::*;

//Components
#[derive(Resource)]
pub struct SpawnTimer(pub Timer);

//Systems
pub fn spawn_enemy(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut timer: ResMut<SpawnTimer>,
    ){
    if timer.0.tick(time.delta()).just_finished() {
        let window = window_query.get_single().unwrap();
        let lim_up = window.width() - 85.0;
        let xpos: f32 = rand::thread_rng().gen_range(85.0.. lim_up);
        commands.spawn((SpriteBundle {
            texture: asset_server.load("sprites/ball_red_small.png"),
            transform: Transform::from_xyz(xpos, window.height() - 20.0, 0.0),
            ..default()
        }, Enemy{}));
    }
}
