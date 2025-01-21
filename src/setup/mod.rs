use bevy::prelude::*;
use bevy::app::App;

use crate::events::GameOver;
use crate::{GameState, SimulationState};

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
        .add_systems(Update, exit_game)
        .add_systems(Update,toggle_simulation_state.run_if(in_state(GameState::Game)))
        .add_systems(Update, transision_to_gamestate_system)
        .add_systems(Update,transision_to_main_menu_system);
    }
}

pub fn transision_to_gamestate_system(keyboard_input: Res<ButtonInput<KeyCode>>, current_game_state: Res<State<GameState>>, mut next_game_state: ResMut<NextState<GameState>> ){
    if !current_game_state.get().eq(&GameState::Game) && keyboard_input.just_pressed(KeyCode::KeyG) {
        next_game_state.set(GameState::Game);
        println!("We have entered in game State")
    }
    
}
pub fn transision_to_main_menu_system(keyboard_input: Res<ButtonInput<KeyCode>>, current_game_state: Res<State<GameState>>, mut next_game_state: ResMut<NextState<GameState>>){
    if !current_game_state.get().eq(&GameState::MainMenu) && keyboard_input.just_pressed(KeyCode::Semicolon) {
        next_game_state.set(GameState::Game);
        println!("We have entered in MainMenu state")
    }
}

pub fn toggle_simulation_state(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<SimulationState>>,
    mut next_state: ResMut<NextState<SimulationState>>
) {
    if keyboard_input.just_pressed(KeyCode::Space) && current_state.get().eq(&SimulationState::Running)  {
         next_state.set(SimulationState::Paused); 
         println!("Game Paused");
    }
     if keyboard_input.just_pressed(KeyCode::Space) && current_state.get().eq(&SimulationState::Paused)  {
         next_state.set(SimulationState::Running);
         println!("Game Resumed"); 
    }
}