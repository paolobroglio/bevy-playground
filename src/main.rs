use bevy::prelude::*;

use crate::game::SimulationPlugin;
use crate::main_menu::MainMenuPlugin;
use crate::systems::{exit_game, spawn_camera, transition_to_game_state, transition_to_main_menu_state};

mod systems;
mod game;
mod main_menu;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum ApplicationState {
    #[default]
    MainMenu,
    Game,
    GameOver
}

fn main() {
    App::new()
        // Bevy Plugins
        .add_plugins(DefaultPlugins)
        // States
        .add_state::<ApplicationState>()
        // Game Plugins
        .add_plugin(MainMenuPlugin)
        .add_plugin(SimulationPlugin)
        // Startup Systems
        .add_startup_system(spawn_camera)
        // Systems
        .add_system(transition_to_main_menu_state)
        .add_system(transition_to_game_state)
        .add_system(exit_game)
        .run();
}
