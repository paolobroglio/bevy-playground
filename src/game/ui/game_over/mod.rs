mod layout;
mod components;
mod interactions;

use bevy::prelude::*;
use crate::ApplicationState;
use crate::game::ui::game_over::interactions::{interact_with_main_menu_button, interact_with_quit_button, update_final_score_text};
use crate::game::ui::game_over::layout::{despawn_game_over_menu, spawn_game_over_menu};


pub const NORMAL_BUTTON_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON_COLOR: Color = Color::rgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON_COLOR: Color = Color::rgb(0.35, 0.75, 0.35);


pub struct GameOverPlugin;
impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app
            // OnEnter Systems
            .add_system(spawn_game_over_menu.in_schedule(OnEnter(ApplicationState::GameOver)))
            // Systems
            .add_systems(
                (
                    interact_with_main_menu_button,
                    interact_with_quit_button,
                    update_final_score_text
                    )
                    .in_set(OnUpdate(ApplicationState::GameOver))
            )
            // OnExit Systems
            .add_system(despawn_game_over_menu.in_schedule(OnExit(ApplicationState::GameOver)));
    }
}