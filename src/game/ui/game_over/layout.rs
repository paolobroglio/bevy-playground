use bevy::prelude::*;
use crate::game::ui::game_over::components::{FinalScoreText, GameOverMenu, MainMenuButton, QuitButton};

pub fn spawn_game_over_menu(
    mut commands: Commands, asset_server: Res<AssetServer>
) {
    build_game_over_menu(&mut commands, &asset_server);
}

pub fn despawn_game_over_menu(
    mut commands: Commands,
    pause_menu_query: Query<Entity, With<GameOverMenu>>
) {
    if let Ok(pause_menu_query) = pause_menu_query.get_single() {
        commands.entity(pause_menu_query).despawn_recursive();
    }
}

pub fn build_game_over_menu(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>
) -> Entity {
    let pause_menu_entity = commands.spawn(
        (
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    gap: Size::new(Val::Px(8.0), Val::Px(8.0)),
                    ..default()
                },
                ..default()
            },
            GameOverMenu
        )
    )
        .with_children(|parent| {
            // === Title ===
            parent.spawn(
                NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        size: Size::new(Val::Px(300.0), Val::Px(120.0)),
                        ..default()
                    },
                    ..default()
                }
            ).with_children(|parent|{
                // === Text ===
                parent.spawn(
                    TextBundle {
                        text: Text {
                            sections: vec![
                                TextSection::new(
                                    "GAME OVER!",
                                    TextStyle {
                                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                        font_size: 64.0,
                                        color: Color::WHITE,
                                        ..default()
                                    }
                                )
                            ],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    }
                );
            });
            // === Final Score Text ===
            parent.spawn(
                NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        size: Size::new(Val::Px(300.0), Val::Px(120.0)),
                        ..default()
                    },
                    ..default()
                }
            ).with_children(|parent|{
                // === Text ===
                parent.spawn(
                    (
                        TextBundle {
                            text: Text {
                                sections: vec![
                                    TextSection::new(
                                        "",
                                        TextStyle {
                                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                            font_size: 32.0,
                                            color: Color::WHITE,
                                            ..default()
                                        }
                                    )
                                ],
                                alignment: TextAlignment::Center,
                                ..default()
                            },
                            ..default()
                        },
                        FinalScoreText
                    )
                );
            });
            // === Main Menu Button ===
            parent.spawn(
                (
                    ButtonBundle {
                        style: Style {
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            size: Size::new(Val::Px(200.0), Val::Px(80.0)),
                            ..default()
                        },
                        background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                        ..default()
                    },
                    MainMenuButton
                )
            ).with_children(|parent| {
                parent.spawn(TextBundle {
                    text: Text {
                        sections: vec![
                            TextSection::new(
                                "Main Menu",
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 32.0,
                                    color: Color::WHITE,
                                    ..default()
                                }
                            )
                        ],
                        alignment: TextAlignment::Center,
                        ..default()
                    },
                    ..default()
                });
            });
            // === Quit Button ===
            parent.spawn(
                (
                    ButtonBundle {
                        style: Style {
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            size: Size::new(Val::Px(200.0), Val::Px(80.0)),
                            ..default()
                        },
                        background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                        ..default()
                    },
                    QuitButton
                )
            ).with_children(|parent| {
                parent.spawn(TextBundle {
                    text: Text {
                        sections: vec![
                            TextSection::new(
                                "Quit",
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 32.0,
                                    color: Color::WHITE,
                                    ..default()
                                }
                            )
                        ],
                        alignment: TextAlignment::Center,
                        ..default()
                    },
                    ..default()
                });
            });
        })
        .id();

    pause_menu_entity
}