use bevy::prelude::*;

#[derive(Component)]
pub struct Ball {
    pub direction_x: f32,
    pub direction_y: f32,
}