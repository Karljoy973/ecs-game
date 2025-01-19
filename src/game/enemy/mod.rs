use bevy::app::{App, Plugin, Startup, Update};
use resources::EnemySpawnTimer;

pub mod component; 
pub mod system;
pub mod resources; 

pub const ENEMY_COUNT: usize = 4; 
pub const ENEMY_SIZE: f32 = 64.; 
pub const ENEMY_SPEED: f32 = 480.;
pub const ENEMY_SPAWN_TIME: f32 = 3.;

use crate::game::enemy::system::{enemy_hit_player, spawn_enemies_over_time,tick_enemies_spawn_timer,spawn_enemies, update_enemy_direction, enemy_movement};
pub struct  EnemyPlugin; 

impl Plugin for EnemyPlugin {
    fn build(&self, app:&mut App) {
        app
        .init_resource::<EnemySpawnTimer>()
        .add_systems(Startup, spawn_enemies)
        .add_systems(Update, spawn_enemies_over_time)
        .add_systems(Update, update_enemy_direction)
        .add_systems(Update, enemy_movement)
        .add_systems(Update, enemy_hit_player)
        .add_systems(Update, tick_enemies_spawn_timer);
    }
}