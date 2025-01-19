use bevy::prelude::*;

use crate::events::GameOver;

use super::{component::Score, component::HighScore};

pub fn update_high_score ( mut game_over_event:EventReader<GameOver> , mut high_score: ResMut<HighScore>) {
    for event in  game_over_event.read().into_iter() {
        high_score.scores.push(("Player".to_string(), event.score));
    }
}

pub fn read_high_score (high_score: Res<HighScore>, mut game_over_event:EventReader<GameOver> ) {
    for _ in  game_over_event.read().into_iter() {
        high_score.scores.last().inspect(|x|println!("High Scores : {}", x.1));
    }
}

pub fn update_score (score: Res<Score>) {
    if score.is_changed() {
        println!("Score : {}", score.value);
    }
}
