use bevy::prelude::*;
use crate::events::GameOver;
use crate::states::{ApplicationState, SimulationState};

pub fn handle_game_over(mut game_over_event_reader: EventReader<GameOver>) {
    for event in game_over_event_reader.iter() {
        println!("Your final score is: {}", event.score)
    }
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window>) {
    let window = window_query.get_single().unwrap();
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

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

pub fn transition_to_game_state(
    keyboard_input: Res<Input<KeyCode>>,
    application_state: Res<State<ApplicationState>>,
    mut next_app_state: ResMut<NextState<ApplicationState>>
) {
    if keyboard_input.just_pressed(KeyCode::G) {
        if application_state.0 != ApplicationState::Game {
            next_app_state.set(ApplicationState::Game);
            println!("Application State: Game");
        }
    }
}

pub fn transition_to_main_menu_state(
    keyboard_input: Res<Input<KeyCode>>,
    application_state: Res<State<ApplicationState>>,
    mut next_app_state: ResMut<NextState<ApplicationState>>
) {
    if keyboard_input.pressed(KeyCode::M) {
        if application_state.0 != ApplicationState::MainMenu {
            next_app_state.set(ApplicationState::MainMenu);
            println!("Application State: Main Menu");
        }
    }
}