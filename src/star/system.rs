use bevy::{prelude::*, window::PrimaryWindow};
use rand::random;

use super::{component::Star, resources::StarSpawnTimer, NUMBER_OF_STARS};


pub fn tick_star_spawn_timer(mut star_spawn_timer: ResMut<StarSpawnTimer>, time: Res<Time>) {
    star_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_stars_over_time (
    mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>, asset_server: Res<AssetServer>, star_spawn_timer: Res<StarSpawnTimer>
) {
     let window = match window_query.get_single() {
            Ok(v) => v,
            Err(_) => panic!("No Window found")
        };
    if star_spawn_timer.timer.finished() {
       let random_x = random::<f32>() * window.width()/2.;
            let random_y = random::<f32>() * window.height()/2.;

            commands.spawn( 
                (Sprite::from_image( asset_server.load("sprites/star.png")), 
                Star {}, 
                Transform::from_xyz(random_x, random_y, 0.)));
            
        } 
    
}


pub fn spawn_stars(mut commands: Commands, asset_server: Res<AssetServer>, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = match window_query.get_single() {
            Ok(v) => v,
            Err(_) => panic!("No Window found")
        };
        for _ in 0..=NUMBER_OF_STARS {
            let random_x = random::<f32>() * window.width()/2.;
            let random_y = random::<f32>() * window.height()/2.;

            commands.spawn( 
                (Sprite::from_image( asset_server.load("sprites/star.png")), 
                Star {}, 
                Transform::from_xyz(random_x, random_y, 0.)));
            
        } 
}

