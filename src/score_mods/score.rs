use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::components::*;
use crate::GameState;

pub struct ScorePlugin;

impl Plugin for ScorePlugin{
    fn build(&self, app: &mut App) {
        app.insert_resource(Score{value: 0})
            .add_systems(Update, (score_system, (despawn_previos_score, update_score)
                                  .chain()
                                  .run_if(in_state(GameState::Playing))));
    }
}
const PLAYER_SIZE:f32 = 30.0;
const ENEMY_SIZE:f32 = 20.0;
#[derive(Resource, Debug)]
pub struct Score{
    pub value: u64,
}

#[derive(Component)]
pub struct ScoreText;

fn update_score(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut score: ResMut<Score>,
    ){
    let window = window_query.get_single().unwrap();
    let my_score: String = score.value.to_string();
    //Spawn Camera
    commands.spawn(( TextBundle::from_section(
        my_score,
        TextStyle {
        font: asset_server.load("fonts/mangatb.ttf"),
        font_size: 100.0,
        ..default()},
    ).with_text_justify(JustifyText::Center).with_style(Style {
        position_type: PositionType::Absolute,
        top: Val::Px(window.width() / 2.0),
        right: Val::Px(window.height() / 2.0),
        ..default()
    },), ScoreText{}));
}

fn despawn_previos_score(
    mut commands: Commands,
    query: Query<Entity, With<ScoreText>>,
    ){
    for entity in query.iter(){
        commands.entity(entity).despawn();
    }
}


fn score_system(
    mut commands: Commands,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
    player_query: Query<&Transform, With<Player>>,
    asset_server: Res<AssetServer>,
    mut score: ResMut<Score>,
    ){
    for (entity, transform) in enemy_query.iter(){
        let distance = transform.translation.distance(player_query.single().translation);
        if distance < PLAYER_SIZE + ENEMY_SIZE {
            commands.entity(entity).despawn();
            score.value += 1;
        }
    }
}
