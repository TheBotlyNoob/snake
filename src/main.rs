use bevy::{prelude::*, time::FixedTimestep};
use rand::prelude::*;

#[derive(Component)]
struct SnakeHead {
    direction: Direction,
}
#[derive(Component)]
struct SnakeSegment;

#[derive(Default, Deref, DerefMut, Resource)]
struct SnakeSegments(Vec<Entity>);

const SNAKE_HEAD_COLOR: Color = Color::rgb(0.4, 0.4, 1.);
const SNAKE_BODY_COLOR: Color = Color::rgb(0.4, 1., 0.4);
const FOOD_COLOR: Color = Color::rgb(1., 0.4, 0.4);

const ARENA_HEIGHT: f32 = 10.;
const ARENA_WIDTH: f32 = 10.;

#[derive(Component, Clone, Copy, PartialEq, Eq, Debug)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Size {
    width: f32,
    height: f32,
}
impl Size {
    fn square(s: f32) -> Self {
        Size {
            width: s,
            height: s,
        }
    }
}

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
        .insert(SnakeHead {
            direction: Direction::Right,
        })
        .insert(Position { x: 3, y: 3 })
        .insert(Size::square(0.8));
}

fn scale(windows: Res<Windows>, mut query: Query<(&Size, &mut Transform)>) {
    let Some(window) = windows.get_primary() else {
        return; // something very bad happened... or, the user closed the window
    };

    // this makes the window square, so that the aspect ratio doesn't affect the game
    // if we don't do this, we will get squished sprites
    let window = window.width().min(window.height());

    for (size, mut transform) in &mut query {
        transform.scale = Vec3::new(
            size.width / ARENA_WIDTH * window,
            size.height / ARENA_HEIGHT * window,
            0.8,
        );
    }
}

fn position_translation(windows: Res<Windows>, mut q: Query<(&Position, &mut Transform)>) {
    /// Convert a position in the window to a position in the arena
    ///
    /// # Example
    /// ```
    /// let window_bound = 100.;
    /// let arena_bound = 10.;
    /// let pos = 0.;
    /// let result = conv(pos, window_bound, arena_bound);
    /// assert_eq!(result, -45.);
    /// ```
    fn conv(pos: f32, window_bound: f32, arena_bound: f32) -> f32 {
        let tile = window_bound / arena_bound;
        pos / arena_bound * window_bound - (window_bound / 2.) + (tile / 2.)
    }

    let Some(window) = windows.get_primary() else {
        return; // something very bad happened... or, the user closed the window
    };

    for (pos, mut transform) in &mut q {
        transform.translation = Vec3::new(
            conv(pos.x as _, window.width(), ARENA_WIDTH),
            conv(pos.y as _, window.height(), ARENA_HEIGHT),
            0.,
        );
    }
}

fn snake_movement(
    // mut head_positions: Query<&mut Transform, With<SnakeHead>>,  - we can't use Transform here because the positioning system uses [`Position`] and [`Size`]
    mut head_positions: Query<(&SnakeHead, &mut Position)>,
) {
    for (head, mut pos) in &mut head_positions {
        match head.direction {
            Direction::Up => pos.y += 1,
            Direction::Down => pos.y -= 1,
            Direction::Left => pos.x -= 1,
            Direction::Right => pos.x += 1,
        }
    }
}

fn snake_input(kbd_input: Res<Input<KeyCode>>, mut heads: Query<&mut SnakeHead>) {
    for mut head in &mut heads {
        let dir = if kbd_input.pressed(KeyCode::Up) {
            Direction::Up
        } else if kbd_input.pressed(KeyCode::Down) {
            Direction::Down
        } else if kbd_input.pressed(KeyCode::Left) {
            Direction::Left
        } else if kbd_input.pressed(KeyCode::Right) {
            Direction::Right
        } else {
            return;
        };

        if dir != head.direction.opposite() {
            head.direction = dir;
        }
    }
}

fn spawn_segment(mut commands: Commands, pos: Position) -> Entity {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: SNAKE_BODY_COLOR,
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(10., 10., 10.),
                ..default()
            },
            ..default()
        })
        .insert(SnakeSegment)
        .insert(pos)
        .insert(Size::square(0.8))
        .id()
}

#[derive(Component)]
struct Food;

fn spawn_food(mut commands: Commands) {
    let mut rng = rand::thread_rng();
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: FOOD_COLOR,
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(10., 10., 10.),
                ..default()
            },
            ..default()
        })
        .insert(Food)
        .insert(Position {
            x: rng.gen_range(0..=ARENA_WIDTH as _),
            y: rng.gen_range(0..=ARENA_HEIGHT as _),
        })
        .insert(Size::square(0.8));
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    fn opposite(self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(SnakeSegments::default())
        .add_startup_system(setup_camera)
        .add_startup_system(spawn_snake)
        .add_system(snake_input.before(snake_movement))
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.2))
                .with_system(snake_movement),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(2.))
                .with_system(spawn_food),
        )
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_system(scale)
                .with_system(position_translation),
        )
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Snake".to_string(),
                width: 500.,
                height: 500.,
                ..default()
            },
            ..default()
        }))
        .run();
}
