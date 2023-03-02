use bevy::prelude::*;

use crate::{Position, SnakeHead, SnakeSegments};

#[derive(Default, Debug, Deref, DerefMut, Resource)]
pub(crate) struct LastSegmentPosition(pub Option<Position>);

#[derive(Copy, Clone, Debug, Eq, PartialEq, Resource)]
/// used to store the direction of the snake's head,
/// but it's only updated when the snake moves - not when there is input
/// this is used to prevent the snake from going backwards
pub struct CurrentDirection(pub Direction);

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    const fn opposite(self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}
pub(crate) fn snake(
    segs: Res<SnakeSegments>,
    mut direction: ResMut<CurrentDirection>,
    mut last_segment: ResMut<LastSegmentPosition>,
    // mut head_positions: Query<&mut Transform, With<SnakeHead>>,  - we can't use Transform here because the positioning system uses [`Position`] and [`Size`]
    mut heads: Query<(Entity, &SnakeHead)>,
    mut poses: Query<&mut Position>,
) {
    if let Some((head_entity, head)) = heads.iter_mut().next() {
        let seg_positions = segs
            .iter()
            .map(|e| *poses.get_mut(*e).unwrap())
            .collect::<Vec<Position>>();

        let mut head_pos = poses.get_mut(head_entity).unwrap();
        match &head.direction {
            Direction::Left => {
                head_pos.x -= 1;
            }
            Direction::Right => {
                head_pos.x += 1;
            }
            Direction::Up => {
                head_pos.y += 1;
            }
            Direction::Down => {
                head_pos.y -= 1;
            }
        };
        *direction = CurrentDirection(head.direction);
        *last_segment = LastSegmentPosition(seg_positions.last().copied());

        seg_positions
            .iter()
            .zip(segs.iter().skip(1))
            .for_each(|(pos, seg)| {
                *poses.get_mut(*seg).unwrap() = *pos;
            });
    }
}

pub(crate) fn input(
    kbd_input: Res<Input<KeyCode>>,
    direction: Res<CurrentDirection>,
    mut heads: Query<&mut SnakeHead>,
) {
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

        if dir != direction.0.opposite() {
            head.direction = dir;
        }
    }
}
