mod layout;
mod components;
mod interactions;

use bevy::prelude::*;
use crate::game::SimulationState;
use crate::game::ui::pause_menu::interactions::{interact_with_main_menu_button, interact_with_quit_button, interact_with_resume_button};
use crate::game::ui::pause_menu::layout::{despawn_pause_menu, spawn_pause_menu};


pub const NORMAL_BUTTON_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON_COLOR: Color = Color::rgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON_COLOR: Color = Color::rgb(0.35, 0.75, 0.35);


pub struct PauseMenuPlugin;
impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // OnEnter Systems
            .add_system(spawn_pause_menu.in_schedule(OnEnter(SimulationState::Paused)))
            // Systems
            .add_systems(
                (
                    interact_with_main_menu_button,
                    interact_with_quit_button,
                    interact_with_resume_button
                    )
                    .in_set(OnUpdate(SimulationState::Paused))
            )
            // OnExit Systems
            .add_system(despawn_pause_menu.in_schedule(OnExit(SimulationState::Paused)));
    }
}