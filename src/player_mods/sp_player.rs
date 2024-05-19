use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::components::Player;

pub fn spawn_player(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>, asset_server: Res<AssetServer>) {
    let window = window_query.get_single().unwrap();
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, 50.0, 0.0),
            texture: asset_server.load("sprites/ball_blue_large.png"),
            ..default()
        },Player{}));
}
