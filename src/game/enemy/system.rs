

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::random;
use crate::events::GameOver;
use crate::game::enemy::{component::Enemy, resources::EnemySpawnTimer};
use crate::game::player::component::Player;
use crate::game::player::PLAYER_SIZE;
use crate::game::score::component::Score;
use crate::GameState;

use super::{ENEMY_COUNT, ENEMY_SIZE, ENEMY_SPEED};




pub fn spawn_enemies(mut commands: Commands, asset_server: Res<AssetServer>, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = match window_query.get_single() {
            Ok(v) => v,
            Err(_) => panic!("No Window found")
        };
        for _ in 0..=ENEMY_COUNT {
            let random_x: f32 = random::<f32>() * window.width()/2.;
            let random_y: f32 = random::<f32>() * window.height()/2.;

            commands.spawn( 
                (Sprite::from_image( asset_server.load("sprites/ball_red_large.png")), 
                Enemy {
                    direction: Vec2::new(random::<f32>(),random::<f32>()).normalize()
                }, 
                Transform::from_xyz(random_x, random_y, 0.)));    
        } 
}

pub fn enemy_movement(mut enemy_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>){
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.);
        transform.translation += direction* ENEMY_SPEED* time.delta_secs();
    }

}
pub fn update_enemy_direction(mut enemy_query:Query<(&Transform, &mut Enemy)>, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window: &Window = match window_query.get_single(){
            Ok(v) => v,
            Err(_) => panic!("No Window found")
        };
        for (transform, mut enemy) in enemy_query.iter_mut() {
            let mut direction_changed: bool = false; 
            let half_enemy_size: f32 = ENEMY_SIZE/2.;
            let  translation:Vec3 = transform.translation;  
            let enemy_x_min: f32 =  half_enemy_size - window.width()/2. ;
            let enemy_x_max: f32 = window.width()/2. - half_enemy_size;
            let enemy_y_min: f32 =  half_enemy_size - window.height()/2.;
            let enemy_y_max: f32=   window.height()/2. - half_enemy_size ;

            if  translation.x < enemy_x_min || translation.x > enemy_x_max{ enemy.direction.x *=-1.; direction_changed = true;};
            if  translation.y<enemy_y_min  || translation.y > enemy_y_max {enemy.direction.y *= -1.;direction_changed = true;};

            if direction_changed {}
        }
}

pub fn enemy_hit_player(
    mut commands: Commands, 
    mut player_query: Query<(Entity, &Transform), With<Player>>, 
    enemy_query: Query< &Transform, With<Enemy>>,
    mut next_game_state: ResMut<NextState<GameState>>, 
    score: Res<Score>, 
) {
    let impact_distance = (PLAYER_SIZE+ENEMY_SIZE)/2.;
    //add the audio later 
    let mut finalscore = score.into_inner().value;
    
    if let Ok(( player_entity, player_transform)) = player_query.get_single_mut() {
        for enemy_transform in enemy_query.iter() {
            let distance = player_transform.translation.distance(enemy_transform.translation);
            if distance <impact_distance {
                commands.entity(player_entity).despawn();
                println!("You scored : {}", finalscore);
                next_game_state.set(GameState::GameOver);
                // game_over_event_writer.send(GameOver {score : score.value});
            }
        }
    }

}

pub fn spawn_enemies_over_time(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>, asset_server: Res<AssetServer>, enemy_spawn_timer: Res<EnemySpawnTimer>) {
     let window = match window_query.get_single() {
            Ok(v) => v,
            Err(_) => panic!("No Window found")
        };
    if enemy_spawn_timer.timer.finished() {

            let random_x: f32 = random::<f32>() * window.width()/2.;
            let random_y: f32 = random::<f32>() * window.height()/2.;

            commands.spawn( 
                (Sprite::from_image( asset_server.load("sprites/ball_red_large.png")), 
                Enemy {
                    direction: Vec2::new(random::<f32>(),random::<f32>()).normalize()
                }, 
                Transform::from_xyz(random_x, random_y, 0.)));   
            
        } 
}

pub fn tick_enemies_spawn_timer (mut enemy_spawn_timer: ResMut<EnemySpawnTimer>, time: Res<Time>)
{
    enemy_spawn_timer.timer.tick(time.delta());
}

pub fn despanw_enemies(
    mut commands: Commands, 
    mut enemies_query: Query<(Entity, &Transform), With<Enemy>>, 
    enemy_query: Query< &Transform, With<Enemy>>,
    game_state: Res<State<GameState>>
) {
    if game_state.get().eq(&GameState::GameOver) {
        for (enemy, _) in enemies_query.iter() {
            commands.entity(enemy).despawn();
        } 
    }
}