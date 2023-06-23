use bevy::app::AppExit;
use bevy::prelude::*;

use crate::ApplicationState;
use crate::game::events::GameOver;
use crate::game::ui::game_over::components::{FinalScoreText, MainMenuButton, QuitButton};
use crate::game::ui::game_over::{HOVERED_BUTTON_COLOR, NORMAL_BUTTON_COLOR, PRESSED_BUTTON_COLOR};

pub fn interact_with_main_menu_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<MainMenuButton>),
    >,
    mut app_state_next_state: ResMut<NextState<ApplicationState>>
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Clicked => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                app_state_next_state.set(ApplicationState::MainMenu);
            }
            Interaction::Hovered => {
                *background_color = HOVERED_BUTTON_COLOR.into();
            }
            Interaction::None => {
                *background_color = NORMAL_BUTTON_COLOR.into();
            }
        }
    }
}

pub fn interact_with_quit_button(
    mut app_exit_event_writer: EventWriter<AppExit>,
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<QuitButton>),
    >,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Clicked => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                app_exit_event_writer.send(AppExit);
            }
            Interaction::Hovered => {
                *background_color = HOVERED_BUTTON_COLOR.into();
            }
            Interaction::None => {
                *background_color = NORMAL_BUTTON_COLOR.into();
            }
        }
    }
}

pub fn update_final_score_text(
    mut game_over_event_reader: EventReader<GameOver>,
    mut text_query: Query<&mut Text, With<FinalScoreText>>
) {
    for event in game_over_event_reader.iter() {
        println!("received event game over");
        for mut text in text_query.iter_mut() {
            text.sections[0].value = format!("Final Score: {}", event.score.to_string());
        }
    }
}