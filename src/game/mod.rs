use bevy::prelude::*;

use crate::ApplicationState;
use crate::game::ball::BallPlugin;
use crate::game::events::GameOver;
use crate::game::paddle::PaddlePlugin;
use crate::game::score::ScorePlugin;
use crate::game::systems::{handle_game_over, pause_simulation, resume_simulation, toggle_simulation};

pub mod paddle;
pub mod score;
pub mod ball;
mod systems;
pub mod events;


pub const MARGIN: f32 = 20.0;

pub const BALL_SIDE: f32 = 25.0;
pub const BALL_VELOCITY: f32 = 300.0;

pub const PADDLE_X: f32 = 30.0;
pub const PADDLE_Y: f32 = 100.0;
pub const PADDLE_VELOCITY: f32 = 500.0;


#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    Running,
    #[default]
    Paused
}


pub struct SimulationPlugin;
impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app
            // Events
            .add_event::<GameOver>()
            // States
            .add_state::<SimulationState>()
            // OnEnter Systems
            .add_system(pause_simulation.in_schedule(OnEnter(ApplicationState::Game)))
            // Game Plugin
            .add_plugin(BallPlugin)
            .add_plugin(PaddlePlugin)
            .add_plugin(ScorePlugin)
            // Systems
            .add_system(toggle_simulation.run_if(in_state(ApplicationState::Game)))
            .add_system(handle_game_over)
            // OnExit Systems
            .add_system(resume_simulation.in_schedule(OnExit(ApplicationState::Game)));
    }
}
