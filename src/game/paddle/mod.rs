use bevy::prelude::*;

use crate::ApplicationState;
use crate::game::paddle::systems::{collision, despawn_paddle, move_paddle, preserve_paddle_boundaries, spawn_paddle};
use crate::game::SimulationState;

mod components;
pub mod systems;

pub struct PaddlePlugin;

impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(spawn_paddle.in_schedule(OnEnter(ApplicationState::Game)))
            .add_system(despawn_paddle.in_schedule(OnExit(ApplicationState::Game)))
            .add_systems(
                (move_paddle,preserve_paddle_boundaries, collision)
                    .in_set(OnUpdate(ApplicationState::Game))
                    .in_set(OnUpdate(SimulationState::Running))
            );
    }
}