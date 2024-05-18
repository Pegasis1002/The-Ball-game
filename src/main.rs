use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub mod player_mods;
pub mod enemy_mods;
use player_mods::player::PlayerPlugin;
use enemy_mods::enemy::EnemyPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PlayerPlugin)
        .add_plugins(EnemyPlugin)
        .add_systems(Startup, setup)
        .run()
}

fn setup(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    ){
    let window = window_query.get_single().unwrap();
    //Spawn Camera
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });

    //Background Image
    commands.spawn(SpriteBundle {
        texture: asset_server.load("sprites/background_brown.png"),
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, -50.0).with_scale(Vec3::new(window.width(), window.height(), 0.0)),
        ..default()
    });
}
