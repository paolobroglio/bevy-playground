use bevy::prelude::*;

#[derive(Resource)]
pub struct Score {
    pub value: i32
}

impl Default for Score {
    fn default() -> Score {
        Score { value: 0 }
    }
}