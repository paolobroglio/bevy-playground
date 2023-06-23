use bevy::app::AppExit;
use bevy::prelude::*;

use crate::ApplicationState;

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window>) {
    let window = window_query.get_single().unwrap();
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
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

pub fn exit_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit);
    }
}