use bevy::{
    app::{
        App, 
        Plugin, 
        Startup, 
        Update
    }, prelude::{
        in_state, 
        IntoSystemConfigs
    }
};
use resources::StarSpawnTimer;
pub mod component; 
pub mod system;
pub mod resources;

pub const NUMBER_OF_STARS: usize = 10; 
pub const STAR_SIZE: f32 = 30.;
pub const STAR_SPAWN_TIME : f32 = 1.; 

use crate::{
    game::star::system::{
        spawn_stars, 
        spawn_stars_over_time, 
        tick_star_spawn_timer, 
        despawn_stars
        }, 
        GameState
    };
pub struct  StarPlugin; 

impl Plugin for StarPlugin {
    fn build(&self, app:&mut App) {
        app
            .init_resource::<StarSpawnTimer>()
            .add_systems(
                Startup,
                 spawn_stars)
            .add_systems(
                Update, 
                tick_star_spawn_timer
                    .run_if(in_state(GameState::Game)))
            .add_systems(
                Update, 
                spawn_stars_over_time
                .run_if(in_state(GameState::Game)))
            .add_systems(
                Update, 
                despawn_stars
                .run_if(in_state(GameState::GameOver)));
            
    }
}