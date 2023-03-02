#![warn(clippy::pedantic, clippy::nursery)]
#![allow(
    clippy::needless_pass_by_value,
    clippy::redundant_pub_crate,
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation
)]
#![feature(let_chains)]

use bevy::{prelude::*, time::FixedTimestep};

mod food;
mod movement;

const WINDOW_SIZE: f32 = 800.;

const SNAKE_HEAD_COLOR: Color = Color::rgb(0.4, 0.4, 1.);
const SNAKE_BODY_COLOR: Color = Color::rgb(0.4, 1., 0.4);
const FOOD_COLOR: Color = Color::rgb(1., 0.4, 0.4);

const ARENA_HEIGHT: f32 = 10.;
const ARENA_WIDTH: f32 = 10.;

const STARTING_DIRECTION: movement::Direction = movement::Direction::Up;

const MOVEMENT_RATE: f64 = 0.2; // per second

const FOOD_AMOUNT: usize = 3;

#[derive(Component)]
struct SnakeHead {
    direction: movement::Direction,
}
#[derive(Component)]
struct SnakeSegment;

#[derive(Default, Deref, DerefMut, Resource)]
struct SnakeSegments(Vec<Entity>);

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
    const fn square(s: f32) -> Self {
        Self {
            width: s,
            height: s,
        }
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_snake(
    mut commands: Commands,
    mut segs: ResMut<SnakeSegments>,
    mut spawn_food_writer: EventWriter<food::SpawnFoodEvent>,
) {
    *segs = SnakeSegments(vec![
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
                direction: STARTING_DIRECTION,
            })
            .insert(Position { x: 3, y: 3 })
            .insert(Size::square(0.8))
            .id(),
        spawn_segment(commands, Position { x: 3, y: 2 }),
    ]);

    for _ in 0..FOOD_AMOUNT {
        spawn_food_writer.send(food::SpawnFoodEvent);
    }
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
            0.,
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
        (pos / arena_bound).mul_add(window_bound, -(window_bound / 2.)) + (tile / 2.)
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
        .insert(Size::square(0.6))
        .id()
}

fn main() {
    let mut app = App::new();

    app.insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .insert_resource(SnakeSegments::default())
        .insert_resource(movement::CurrentDirection(movement::Direction::Up))
        .insert_resource(movement::LastSegmentPosition::default());

    app.add_event::<food::GrowthEvent>()
        .add_event::<food::SpawnFoodEvent>();

    app.add_startup_system(setup_camera)
        .add_startup_system(spawn_snake);

    app.add_system(movement::input.before(movement::snake))
        .add_system(food::collision)
        .add_system(food::spawn_food_event)
        .add_system(food::growth_event);

    app.add_system_set(
        SystemSet::new()
            .with_run_criteria(FixedTimestep::step(MOVEMENT_RATE))
            .with_system(movement::snake)
            .with_system(food::collision.after(movement::snake)),
    )
    .add_system_set_to_stage(
        CoreStage::PostUpdate,
        SystemSet::new()
            .with_system(scale)
            .with_system(position_translation),
    );

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        window: {
            #[cfg(target_arch = "wasm32")]
            let window = {
                let window = web_sys::window().unwrap();
                window
                    .inner_height()
                    .unwrap()
                    .as_f64()
                    .unwrap()
                    .min(window.inner_width().unwrap().as_f64().unwrap())
            } as f32;
            #[cfg(not(target_arch = "wasm32"))]
            let window = WINDOW_SIZE;

            WindowDescriptor {
                title: "Snake".to_string(),
                width: window,
                height: window,
                resizable: false,

                ..default()
            }
        },
        ..default()
    }));

    app.run();
}
