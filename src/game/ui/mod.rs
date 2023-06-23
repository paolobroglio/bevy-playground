use bevy::prelude::*;
use crate::game::ui::game_over::GameOverPlugin;
use crate::game::ui::pause_menu::PauseMenuPlugin;

mod pause_menu;
mod game_over;


pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(GameOverPlugin)
            .add_plugin(PauseMenuPlugin);
    }
}