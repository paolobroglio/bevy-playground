use bevy::prelude::*;

use crate::game::events::GameOver;
use crate::game::SimulationState;

pub fn toggle_simulation(
    keyboard_input: Res<Input<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
    mut next_sim_state: ResMut<NextState<SimulationState>>
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if simulation_state.0 != SimulationState::Paused {
            next_sim_state.set(SimulationState::Paused);
            println!("Simulation Paused");
        }
        if simulation_state.0 != SimulationState::Running {
            next_sim_state.set(SimulationState::Running);
            println!("Simulation Running");
        }
    }
}

pub fn pause_simulation(mut next_sim_state: ResMut<NextState<SimulationState>>) {
    next_sim_state.set(SimulationState::Paused);
}

pub fn resume_simulation(mut next_sim_state: ResMut<NextState<SimulationState>>) {
    next_sim_state.set(SimulationState::Running);
}

pub fn handle_game_over(mut game_over_event_reader: EventReader<GameOver>) {
    for event in game_over_event_reader.iter() {
        println!("Your final score is: {}", event.score)
    }
}