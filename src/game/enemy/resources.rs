use bevy::prelude::{Resource, Timer, TimerMode};

use super::ENEMY_SPAWN_TIME;

#[derive(Resource)]
pub struct EnemySpawnTimer {
    pub timer: Timer
}

impl Default for EnemySpawnTimer {
    fn default() -> EnemySpawnTimer {
        EnemySpawnTimer { timer : Timer::from_seconds(ENEMY_SPAWN_TIME, TimerMode::Repeating)}
    }
}