use bevy::prelude::*;
use rand::prelude::*;

use crate::{
    movement::LastSegmentPosition, spawn_segment, Position, Size, SnakeHead, SnakeSegments,
    ARENA_HEIGHT, ARENA_WIDTH, FOOD_COLOR,
};

#[derive(Component)]
pub struct Food;

pub(crate) fn collision(
    mut commands: Commands,
    mut eaten_writer: EventWriter<GrowthEvent>,
    mut food: Query<(Entity, &Position), With<Food>>,
    snake: Query<&Position, With<SnakeHead>>,
) {
    for food in food.iter_mut() {
        if snake.iter().any(|pos| pos == food.1) {
            commands.entity(food.0).despawn();
            eaten_writer.send(GrowthEvent);
        }
    }
}

pub struct GrowthEvent;

pub(crate) fn growth_event(
    commands: Commands,
    last_segment: Res<LastSegmentPosition>,
    mut spawn_food_writer: EventWriter<SpawnFoodEvent>,
    mut growth_events: EventReader<GrowthEvent>,
    segs: ResMut<SnakeSegments>,
) {
    if let Some(pos) = last_segment.0 && growth_events.iter().next().is_some() {
        spawn_segment(commands, segs, pos);
        spawn_food_writer.send(SpawnFoodEvent);
    }
}

pub struct SpawnFoodEvent;

pub(crate) fn spawn_food_event(
    mut commands: Commands,
    mut spawn_food_events: EventReader<SpawnFoodEvent>,
) {
    for _ in spawn_food_events.iter() {
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
            .insert(dbg!(Position {
                x: rng.gen_range(0..=ARENA_WIDTH as _),
                y: rng.gen_range(0..=ARENA_HEIGHT as _),
            }))
            .insert(Size::square(0.4));
    }
}
