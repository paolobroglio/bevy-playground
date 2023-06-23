use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::Rng;

use crate::game::{BALL_SIDE, MARGIN, PADDLE_VELOCITY, PADDLE_X, PADDLE_Y};
use crate::game::ball::components::Ball;
use crate::game::paddle::components::Paddle;
use crate::game::score::resources::Score;

pub fn move_paddle(
    keyboard_input: Res<Input<KeyCode>>,
    mut paddle_query: Query<&mut Transform, With<Paddle>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = paddle_query.get_single_mut() {

        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Down) {
            direction += Vec3::new(0.0, -1.0, 0.0)
        }
        if keyboard_input.pressed(KeyCode::Up) {
            direction += Vec3::new(0.0, 1.0, 0.0)
        }

        direction = direction.normalize_or_zero();

        transform.translation += direction * PADDLE_VELOCITY * time.delta_seconds();
    }
}

pub fn preserve_paddle_boundaries(
    mut paddle_query: Query<&mut Transform, With<Paddle>>,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    if let Ok(mut paddle) = paddle_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        let half_paddle_y_size = PADDLE_Y / 2.0;

        let upper_limit = window.height() - MARGIN - half_paddle_y_size;
        let lower_limit = MARGIN + half_paddle_y_size;

        let mut translation = paddle.translation;

        if translation.y >= upper_limit {
            translation.y = upper_limit;
        }
        if translation.y <= lower_limit {
            translation.y = lower_limit;
        }

        paddle.translation = translation;
    }
}

pub fn spawn_paddle(mut commands: Commands, window_query: Query<&Window>) {
    let window = window_query.get_single().unwrap();

    let rect_x = MARGIN + (PADDLE_X / 2.0);
    let rect_y = window.height() / 2.0;

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1.0, 1.0, 1.0),
                custom_size: Some(Vec2::new(PADDLE_X, PADDLE_Y)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(rect_x, rect_y, 0.0)),
            ..default()
        },
        Paddle
    ));
}

pub fn despawn_paddle(mut commands: Commands, paddle_query: Query<Entity, With<Paddle>>) {
    for paddle_entity in paddle_query.iter() {
        commands.entity(paddle_entity).despawn();
    }
}

pub fn collision(
    mut ball_query: Query<(&Transform, &mut Ball)>,
    paddle_query: Query<&Transform, (With<Paddle>, Without<Ball>)>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
    mut score: ResMut<Score>
) {
    if let Ok((transform, mut ball)) = ball_query.get_single_mut() {
        for paddle in paddle_query.iter() {
            let distance = paddle.translation.distance(transform.translation);
            let paddle_half_x = PADDLE_X / 2.0;
            let ball_half_side = BALL_SIDE / 2.0;

            if distance < paddle_half_x + ball_half_side {
                ball.direction_x = 1.0;
                let mut rand = rand::thread_rng();
                if rand.gen_range(0..3) % 2 == 0 {
                    ball.direction_y = 1.0;
                }
                let bounce_sound_effect = asset_server.load("audio/bounce.ogg");
                audio.play(bounce_sound_effect);
                score.value += 1
            }
        }
    }
}