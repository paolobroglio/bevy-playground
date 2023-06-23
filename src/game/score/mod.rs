use bevy::prelude::*;

use crate::ApplicationState;
use crate::game::score::systems::{insert_score, remove_score, update_score};

pub mod resources;
pub mod systems;


pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(insert_score.in_schedule(OnEnter(ApplicationState::Game)))
            .add_system(remove_score.in_schedule(OnExit(ApplicationState::Game)))
            .add_system(update_score.run_if(in_state(ApplicationState::Game)));
    }
}