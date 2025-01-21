use bevy::prelude::*;
use game::{enemy::EnemyPlugin, player::PlayerPlugin, score::ScorePlugin,  star::StarPlugin};
mod setup;
mod events;
use events::GameOverPlugin;
use setup::SetupPlugin;


mod game;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<SimulationState>()
        .init_state::<GameState>()
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
#[derive(States ,Debug, Clone, Copy, Hash, Default)]
pub enum GameState {
    #[default]
    MainMenu, 
    Game, 
    GameOver
}


#[derive(Eq)]
#[derive(PartialEq)]
#[derive(States, Debug, Clone, Copy, Hash, Default)]
pub enum SimulationState {
    Running, 
    #[default]
    Paused
}