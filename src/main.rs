mod events;
mod ball;
mod paddle;
mod score;
mod systems;
mod constants;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use crate::ball::systems::{move_ball, preserve_ball_boundaries, spawn_ball};
use crate::events::GameOver;
use crate::paddle::systems::{collision, move_paddle, preserve_paddle_boundaries, spawn_paddle};
use crate::score::resources::Score;
use crate::score::systems::update_score;
use crate::systems::{handle_game_over, spawn_camera};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(spawn_paddle)
        .add_startup_system(spawn_ball)
        .add_startup_system(spawn_camera)
        .init_resource::<Score>()
        .add_event::<GameOver>()
        .add_system(update_score)
        .add_system(move_ball)
        .add_system(move_paddle)
        .add_system(preserve_paddle_boundaries)
        .add_system(preserve_ball_boundaries)
        .add_system(collision)
        .add_system(handle_game_over)
        .run();
}
