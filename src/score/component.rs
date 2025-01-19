use bevy::{math::Vec2, prelude:: Resource}; 

#[derive(Resource)]
pub struct Score {
   pub value: u32
}

#[derive(Resource)]
pub struct HighScore {
    pub scores: Vec<(String, u32)>
}
