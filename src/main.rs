use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub mod player_mods;
pub mod enemy_mods;
pub mod env_mods;
pub mod components;
pub mod score_mods;
use player_mods::player::PlayerPlugin;
use enemy_mods::enemy::EnemyPlugin;
use env_mods::env::EnvPlugin;
use score_mods::score::ScorePlugin;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    Playing,
    GameOver,
}

fn main() {
    Game();
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
}

fn background(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    ) {
    let window = window_query.get_single().unwrap();
    let width = window.width();
    commands.spawn(
        SpriteBundle {
            texture: asset_server.load("sprites/background_brown.png"),
            transform: Transform::from_xyz(width / 2.0, window.height()/2.0, -10.0).with_scale(Vec3::new (window.width()/2.0, window.height()/2.0, 2.0)),
            ..default()
        }
    );
}


fn Game(){
        App::new()
            .init_state::<GameState>()
            .add_plugins(DefaultPlugins)
            .add_plugins(PlayerPlugin)
            .add_plugins(EnemyPlugin)
            .add_plugins(EnvPlugin)
            .add_plugins(ScorePlugin)
            .add_systems(Startup, (setup, background))
            .run()
}
