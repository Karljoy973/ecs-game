use bevy::prelude::*;
use enemy::EnemyPlugin;
use events::GameOverPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use setup::SetupPlugin;
use star::StarPlugin;

mod events;
mod enemy;
mod player; 
mod score;
mod star;
mod setup;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GameOverPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(StarPlugin)
        .add_plugins(SetupPlugin)
        .add_plugins(EnemyPlugin)
        .add_plugins(ScorePlugin)
        .run();
}


