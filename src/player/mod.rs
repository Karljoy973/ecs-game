pub mod component; 
pub mod system;
pub mod resources;

use bevy::{app::{App, Plugin, Startup, Update}, prelude::IntoSystemConfigs};
pub const PLAYER_SIZE: f32 = 64.;
pub const PLAYER_SPEED: f32 = 500.;

use crate::player::system::{confine_player_mouvement, spawn_player, player_mouvement, player_catch_star};
pub struct  PlayerPlugin; 

impl Plugin for PlayerPlugin {
    fn build(&self, app:&mut App) {
        app
            .add_systems(Startup, spawn_player)
            .add_systems(Update, confine_player_mouvement)
            .add_systems(Update, player_mouvement.before(confine_player_mouvement))
            .add_systems(Update, player_catch_star);
    }
}