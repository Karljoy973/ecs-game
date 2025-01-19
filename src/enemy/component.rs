use bevy::{math::Vec2, prelude::Component}; 

#[derive(Component)]
pub struct Enemy {
    pub direction: Vec2, 

}