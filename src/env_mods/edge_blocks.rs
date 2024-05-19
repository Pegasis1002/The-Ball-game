use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn edge_blocks(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    ) {
    let window = window_query.get_single().unwrap();
    let width = window.width();
    let height = window.height();
    //left side terminal blocks
    commands.spawn((SpriteBundle {
                texture: asset_server.load("sprites/block_square.png"),
                transform: Transform::from_xyz(30.0, 30.0, -5.0),
                ..default()
            },
            ));
    commands.spawn((SpriteBundle {
                texture: asset_server.load("sprites/block_square.png"),
                transform: Transform::from_xyz(30.0, height - 30.0, -5.0),
                ..default()
            },
            ));

    commands.spawn(
        SpriteBundle{
            texture: asset_server.load("sprites/block_narrow.png"),
            transform: Transform::from_xyz(30.0, height / 2.0, -10.0).with_scale(Vec3::new (1.0, 30.0, 1.0)),
            ..default()
        }
    );
    //right side terminal blocks
    commands.spawn(
        SpriteBundle{
            texture: asset_server.load("sprites/block_square.png"),
            transform: Transform::from_xyz(width - 30.0, 30.0, -5.0),
            ..default()
        });
    commands.spawn(
        SpriteBundle{
            texture: asset_server.load("sprites/block_square.png"),
            transform: Transform::from_xyz(width - 30.0, height - 30.0, -5.0),
            ..default()
        });
    commands.spawn(
        SpriteBundle{
            texture: asset_server.load("sprites/block_narrow.png"),
            transform: Transform::from_xyz(width - 30.0, height / 2.0, -10.0).with_scale(Vec3::new (1.0, 30.0, 1.0)),
            ..default()
        }
    );
}
