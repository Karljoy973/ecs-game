use bevy::{app::{App, Plugin, Startup, Update}, prelude::{in_state, IntoSystemConfigs}};
use resources::EnemySpawnTimer;

pub mod component; 
pub mod system;
pub mod resources; 

pub const ENEMY_COUNT: usize = 4; 
pub const ENEMY_SIZE: f32 = 64.; 
pub const ENEMY_SPEED: f32 = 480.;
pub const ENEMY_SPAWN_TIME: f32 = 3.;

use crate::{game::enemy::system::{enemy_hit_player, enemy_movement, spawn_enemies, spawn_enemies_over_time, tick_enemies_spawn_timer, update_enemy_direction, despanw_enemies}, GameState};
pub struct  EnemyPlugin; 

impl Plugin for EnemyPlugin {
    fn build(&self, app:&mut App) {
        app
        .init_resource::<EnemySpawnTimer>()
        .add_systems(Startup, spawn_enemies)
        .add_systems(Update, spawn_enemies_over_time.run_if(in_state(GameState::Game)) )
        .add_systems(Update, update_enemy_direction)
        .add_systems(Update, enemy_movement.run_if(in_state(GameState::Game)))
        .add_systems(Update, enemy_hit_player.run_if(in_state(GameState::Game)))
        .add_systems(Update, tick_enemies_spawn_timer.run_if(in_state(GameState::Game)))
        .add_systems(Update, despanw_enemies.run_if(in_state(GameState::GameOver)));
    }
}