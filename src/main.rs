use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::Rng;

const MARGIN: f32 = 20.0;

const PADDLE_X: f32 = 30.0;
const PADDLE_Y: f32 = 100.0;

const BALL_SIDE: f32 = 25.0;

const BALL_VELOCITY: f32 = 300.0;
const PADDLE_VELOCITY: f32 = 500.0;

#[derive(Component)]
struct Ball {
    direction_x: f32,
    direction_y: f32,
}

#[derive(Component)]
struct Paddle;

#[derive(Resource)]
struct Score {
    value: i32
}

impl Default for Score {
    fn default() -> Score {
        Score { value: 0 }
    }
}

fn setup(mut commands: Commands, window_query: Query<&Window>) {
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

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

fn move_ball(
    mut ball_query: Query<(&mut Transform, &Ball)>,
    time: Res<Time>
) {
    if let Ok((mut transform, ball)) = ball_query.get_single_mut() {
        let direction = Vec3::new(ball.direction_x, ball.direction_y, 0.0).normalize();

        transform.translation += direction * BALL_VELOCITY * time.delta_seconds();
    }
}

fn move_paddle(
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

fn preserve_paddle_boundaries(
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

fn preserve_ball_boundaries(
    mut ball_query: Query<(&Transform, &mut Ball)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>
) {
    if let Ok((transform, mut ball)) = ball_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        let half_ball_size = BALL_SIDE / 2.0;

        let left_limit = MARGIN + half_ball_size;
        let right_limit = window.width() - MARGIN - half_ball_size;
        let upper_limit = window.height() - MARGIN - half_ball_size;
        let lower_limit = MARGIN + half_ball_size;

        let translation = transform.translation;

        //if translation.x <= left_limit {
        //   println!("YOU LOST!");
        //}
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

fn collision(
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

fn update_score(score: Res<Score>) {
    if score.is_changed() {
        println!("Score: {}", score.value.to_string());
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(setup)
        .init_resource::<Score>()
        .add_system(update_score)
        .add_system(move_ball)
        .add_system(move_paddle)
        .add_system(preserve_paddle_boundaries)
        .add_system(preserve_ball_boundaries)
        .add_system(collision)
        .run();
}
