use bevy::prelude::*;
use game::{enemy::EnemyPlugin, player::PlayerPlugin, score::ScorePlugin, setup::SetupPlugin, star::StarPlugin};
mod events;
use events::{GameOverPlugin, UpdateState};


mod game;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state_scoped_event::<UpdateState>(GameState::MainMenu)
        .add_state_scoped_event::<UpdateState>(SimulationState::Paused)
        .add_plugins(GameOverPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(StarPlugin)
        .add_plugins(SetupPlugin)
        .add_plugins(EnemyPlugin)
        .add_plugins(ScorePlugin)
        .run();
}

#[derive(Eq)]
#[derive(PartialEq)]
#[derive(States,Event ,Debug, Clone, Copy, Hash, Default)]
pub enum GameState {
    #[default]
    MainMenu, 
    Game, 
    GameOver
}


#[derive(Eq)]
#[derive(PartialEq)]
#[derive(States,Event, Debug, Clone, Copy, Hash, Default)]
pub enum SimulationState {
    Running, 
    #[default]
    Paused
}