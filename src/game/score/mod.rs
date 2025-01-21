use bevy::app::{App, Last, Plugin, Update};
use component::{HighScore, Score};
use crate::game::score::system::{read_high_score, update_high_score, update_score};

pub mod component; 
pub mod system;


pub struct ScorePlugin; 

impl Plugin for ScorePlugin {
    fn build(&self, app:&mut App) {
        app
            .init_resource::<Score>()
            .init_resource::<HighScore>()
            .add_systems(Update, update_score)
            .add_systems(Update, update_high_score)
            .add_systems(Last, read_high_score);
    }
}