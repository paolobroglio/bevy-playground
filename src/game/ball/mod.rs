use bevy::prelude::*;

use crate::ApplicationState;
use crate::game::ball::systems::{despawn_ball, move_ball, preserve_ball_boundaries, spawn_ball};
use crate::game::SimulationState;

pub mod components;
pub mod systems;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(spawn_ball.in_schedule(OnEnter(ApplicationState::Game)))
            .add_systems(
                (move_ball, preserve_ball_boundaries)
                    .in_set(OnUpdate(ApplicationState::Game))
                    .in_set(OnUpdate(SimulationState::Running))
            )
            .add_system(despawn_ball.in_schedule(OnExit(ApplicationState::Game)));

    }
}