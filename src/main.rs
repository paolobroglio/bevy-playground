use bevy::prelude::*;

use crate::ball::BallPlugin;
use crate::events::GameOver;
use crate::paddle::PaddlePlugin;
use crate::score::ScorePlugin;
use crate::states::{ApplicationState, SimulationState};
use crate::systems::{handle_game_over, spawn_camera, toggle_simulation, transition_to_game_state, transition_to_main_menu_state};

mod events;
mod ball;
mod paddle;
mod score;
mod systems;
mod constants;
mod states;

fn main() {
    App::new()
        // Bevy Plugins
        .add_plugins(DefaultPlugins)
        // States
        .add_state::<ApplicationState>()
        .add_state::<SimulationState>()
        // Game Plugin
        .add_plugin(BallPlugin)
        .add_plugin(PaddlePlugin)
        .add_plugin(ScorePlugin)
        // Events
        .add_event::<GameOver>()
        // Startup Systems
        .add_startup_system(spawn_camera)
        // Systems
        .add_system(handle_game_over)
        .add_system(toggle_simulation.run_if(in_state(ApplicationState::Game)))
        .add_system(transition_to_main_menu_state)
        .add_system(transition_to_game_state)
        .run();
}
