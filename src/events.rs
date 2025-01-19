use bevy::{app::{App, Plugin}, prelude::Event};
#[derive(Event)]
pub struct GameOver {
    pub score : u32
}

pub struct GameOverPlugin ; 

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GameOver>();
    }
}
#[derive(Event)]
pub struct UpdateState ;