use bevy::{prelude::*, window::PrimaryWindow};

use crate::{score::component::Score, star::{component::Star, NUMBER_OF_STARS, STAR_SIZE}};

use super::{component::Player, PLAYER_SIZE, PLAYER_SPEED};


pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Sprite::from_image(
        asset_server.load("sprites/ball_blue_large.png"),
    ), Player {}));
}



pub fn player_mouvement( keyboard_input: Res<ButtonInput<KeyCode>>, mut player_query: Query<&mut Transform, With<Player>>, time: Res<Time>){
    if let Ok(mut transform) = player_query.get_single_mut(){
        let mut direction = Vec3::ZERO; 
        if keyboard_input.pressed(KeyCode::KeyW) {
            direction += Vec3::new(0.,1.,0.)
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            direction -= Vec3::new(1.,0.,0.)
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            direction -= Vec3::new(0.,1.,0.)
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            direction += Vec3::new(1.,0.,0.)
        }

        if direction.length() > 0.00 {
            direction = direction.normalize();
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_secs();

    }
}

pub fn confine_player_mouvement(mut player_query: Query<&mut Transform, With<Player>>, window_query: Query<&Window, With<PrimaryWindow>>) {
    if let Ok(mut player_transform) = player_query.get_single_mut(){
        let window = match window_query.get_single() {
            Ok(v) => v,
            Err(_) => panic!("No Window found")
        };
        let half_player_size: f32 = PLAYER_SIZE/2.;
        let player_x_min: f32 =  half_player_size - window.width()/2. ;
        let player_x_max: f32 = window.width()/2. - half_player_size;
        let player_y_min: f32 =  half_player_size - window.height()/2.;
        let player_y_max: f32=   window.height()/2. - half_player_size ;

        let mut translation:Vec3 = player_transform.translation; 
        if translation.x < player_x_min { translation.x = player_x_min;}
        else if translation.x > player_x_max {translation.x = player_x_max;}
        if translation.y < player_y_min {translation.y = player_y_min;}
        else if translation.y > player_y_max {translation.y = player_y_max;}
        player_transform.translation = translation; 
    }
}



pub fn player_catch_star (mut commands: Commands, 
    mut player_query: Query<(Entity, &Transform), 
    With<Player>>, 
    star_query: Query<(Entity, &Transform), 
    With<Star>>,
    mut score : ResMut<Score>
) {
    let impact_distance = (STAR_SIZE+PLAYER_SIZE)/2.;
    //add the audio later 
    if let Ok(( _, player_transform)) = player_query.get_single_mut() {
        for (star_entity, star_transform) in star_query.iter() {
            let distance = player_transform.translation.distance(star_transform.translation);
            if distance <impact_distance {
                commands.entity(star_entity).despawn();
                score.value += 1;
            }
        }
    }
}
