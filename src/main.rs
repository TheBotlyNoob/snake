use bevy::prelude::*;

#[derive(Component)]
struct SnakeHead;

const SNAKE_HEAD_COLOR: Color = Color::rgb(0.4, 0.4, 1.);

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_snake(mut commands: Commands) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: SNAKE_HEAD_COLOR,
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(10., 10., 10.),
                ..default()
            },
            ..default()
        })
        .insert(SnakeHead);
}

fn snake_movement(
    kbd_input: Res<Input<KeyCode>>,
    mut head_positions: Query<&mut Transform, With<SnakeHead>>,
) {
    for mut transformer in &mut head_positions {
        let t = &mut transformer.translation;
        if kbd_input.pressed(KeyCode::Left) {
            t.x -= 2.;
        }
        if kbd_input.pressed(KeyCode::Right) {
            t.x += 2.;
        }
        if kbd_input.pressed(KeyCode::Up) {
            t.y += 2.;
        }
        if kbd_input.pressed(KeyCode::Down) {
            t.y -= 2.;
        }
    }
}

fn main() {
    App::new()
        .add_startup_system(setup_camera)
        .add_startup_system(spawn_snake)
        .add_plugins(DefaultPlugins)
        .add_system(snake_movement)
        .run();
}
