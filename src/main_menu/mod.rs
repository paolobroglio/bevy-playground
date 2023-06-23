mod layout;
mod components;
mod interactions;

use bevy::prelude::*;
use crate::ApplicationState;
use crate::main_menu::interactions::{interact_with_play_button, interact_with_quit_button};
use crate::main_menu::layout::{despawn_main_menu, spawn_main_menu};


pub const NORMAL_BUTTON_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON_COLOR: Color = Color::rgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON_COLOR: Color = Color::rgb(0.35, 0.75, 0.35);


pub struct MainMenuPlugin;
impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // OnEnter Systems
            .add_system(spawn_main_menu.in_schedule(OnEnter(ApplicationState::MainMenu)))
            // Systems
            .add_systems(
                (
                    interact_with_play_button,
                    interact_with_quit_button
                    )
                    .in_set(OnUpdate(ApplicationState::MainMenu))
            )
            // OnExit Systems
            .add_system(despawn_main_menu.in_schedule(OnExit(ApplicationState::MainMenu)));
    }
}