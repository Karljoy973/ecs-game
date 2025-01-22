pub mod component; 
pub mod system;
pub mod resources;

use bevy::{
    app::{
        App, 
        Plugin, 
        Startup, 
        Update
    }, 
    prelude::{
        in_state, 
        IntoSystemConfigs
    }
};
use crate::{
        game::player::system::{
        confine_player_mouvement, 
        player_catch_star, 
        player_mouvement, 
        spawn_player
    }, 
    GameState
};

pub const PLAYER_SIZE: f32 = 64.;
pub const PLAYER_SPEED: f32 = 500.;

pub struct  PlayerPlugin; 

impl Plugin for PlayerPlugin {
    fn build(&self, app:&mut App) {
        app
            .add_systems(Startup, spawn_player)
            .add_systems(Update, confine_player_mouvement.after(player_mouvement))
            .add_systems(Update, player_mouvement.run_if(in_state(GameState::Game)))
            .add_systems(Update, player_catch_star);
    }
}