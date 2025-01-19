use bevy::{ prelude::*, window::PrimaryWindow};
use enemy::resources::EnemySpawnTimer;
use score::component::{HighScore, Score};
use star::resources::StarSpawnTimer;

mod events;
mod enemy;
mod player; 
mod score;
mod star;
mod setup;


use crate::events::GameOver;
use crate::enemy::system::{enemy_hit_player, spawn_enemies_over_time,tick_enemies_spawn_timer,spawn_enemies, update_enemy_direction, enemy_movement};
use crate::score::system::{read_high_score, update_high_score, update_score};
use crate::star::system::{spawn_stars_over_time, tick_star_spawn_timer, spawn_stars};
use crate::setup::{exit_game, spawn_camera};
use crate::player::system::{confine_player_mouvement, spawn_player, player_mouvement, player_catch_star};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Score>()
        .init_resource::<StarSpawnTimer>()
        .init_resource::<EnemySpawnTimer>()
        .init_resource::<HighScore>()
        .add_event::<GameOver>()
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_enemies)
        .add_systems(Startup, spawn_player)
        .add_systems(Startup, spawn_stars)
        .add_systems(Update, confine_player_mouvement)
        .add_systems(Update, player_mouvement)
        .add_systems(Update, update_enemy_direction)
        .add_systems(Update, enemy_movement)
        .add_systems(Update, enemy_hit_player)
        .add_systems(Update, player_catch_star)
        .add_systems(Update, update_score)
        .add_systems(Update, tick_star_spawn_timer)
        .add_systems(Update, tick_enemies_spawn_timer)
        .add_systems(Update, spawn_stars_over_time)
        .add_systems(Update, spawn_enemies_over_time)
        .add_systems(Update, update_high_score)
        .add_systems(Update, exit_game)
        .add_systems(Last, read_high_score)
        .run();
}


