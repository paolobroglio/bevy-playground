use bevy::prelude::*;

const MARGIN: f32 = 20.0;


fn setup(mut commands: Commands, window_query: Query<&Window>) {
    let window = window_query.get_single().unwrap();

    let rect_x = MARGIN + 25.0;
    let rect_y = MARGIN + 50.0;

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(1.0, 1.0, 1.0),
            custom_size: Some(Vec2::new(50.0, 100.0)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(rect_x, rect_y, 0.0)),
        ..default()
    });

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_startup_system(setup)
    .run();
}
