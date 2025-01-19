use bevy::prelude::*;
use bevy::app::App;

use crate::events::GameOver;

pub fn spawn_camera(mut commands: Commands) {
       commands.spawn((
        Camera2d,
        Camera::default()
    ));
}


pub fn exit_game (keyboard_input: Res<ButtonInput<KeyCode>>, mut app_exit_event_writer: EventWriter<AppExit>,  game_over_event: EventReader<GameOver>) {
    if keyboard_input.pressed(KeyCode::Escape)  {app_exit_event_writer.send(AppExit::Success);}
    if !game_over_event.is_empty() {app_exit_event_writer.send(AppExit::Success);}
}


pub struct  SetupPlugin; 

impl Plugin for SetupPlugin {
    fn build(&self, app:&mut App) {
        app
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, exit_game);
    }
}