use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::ball::components::Ball;
use crate::constants::{BALL_SIDE, BALL_VELOCITY, MARGIN};
use crate::events::GameOver;
use crate::score::resources::Score;

pub fn move_ball(
    mut ball_query: Query<(&mut Transform, &Ball)>,
    time: Res<Time>
) {
    if let Ok((mut transform, ball)) = ball_query.get_single_mut() {
        let direction = Vec3::new(ball.direction_x, ball.direction_y, 0.0).normalize();

        transform.translation += direction * BALL_VELOCITY * time.delta_seconds();
    }
}

pub fn preserve_ball_boundaries(
    mut game_over_event_writer: EventWriter<GameOver>,
    mut ball_query: Query<(&Transform, &mut Ball)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
    score: Res<Score>
) {
    if let Ok((transform, mut ball)) = ball_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        let half_ball_size = BALL_SIDE / 2.0;

        let left_limit = MARGIN + half_ball_size;
        let right_limit = window.width() - MARGIN - half_ball_size;
        let upper_limit = window.height() - MARGIN - half_ball_size;
        let lower_limit = MARGIN + half_ball_size;

        let translation = transform.translation;

        if translation.x <= left_limit {
            game_over_event_writer.send(GameOver {score: score.value});
        }
        if translation.x >= right_limit {
            let bounce_sound_effect = asset_server.load("audio/bounce.ogg");
            audio.play(bounce_sound_effect);
            ball.direction_x = -1.0;
        }
        if translation.y >= upper_limit {
            let bounce_sound_effect = asset_server.load("audio/bounce.ogg");
            audio.play(bounce_sound_effect);
            ball.direction_y = -1.0;
        }
        if translation.y <= lower_limit {
            let bounce_sound_effect = asset_server.load("audio/bounce.ogg");
            audio.play(bounce_sound_effect);
            ball.direction_y = 1.0;
        }
    }
}

pub fn spawn_ball(mut commands: Commands, window_query: Query<&Window>) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1.0, 0.0, 0.0),
                custom_size: Some(Vec2::new(25.0, 25.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(window.width() / 2.0, window.height() / 2.0, 0.0)),
            ..default()
        },
        Ball {
            direction_x: -1.0,
            direction_y: 0.0
        }
    ));
}

pub fn despawn_ball(mut commands: Commands, ball_query: Query<Entity, With<Ball>>) {
    for ball_entity in ball_query.iter() {
        commands.entity(ball_entity).despawn();
    }
}