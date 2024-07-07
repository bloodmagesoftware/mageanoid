/*
 * Mageanoid - A computer game
 * Copyright (C) 2024  Frank Mayer
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use bevy::math::f32;
use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Velocity {
    pub direction: Vec3,
    pub speed: f32,
}

impl Velocity {
    pub fn from_vec3(direction: Vec3, speed: f32) -> Self {
        Self { direction, speed }
    }

    pub fn from_vec2(direction: Vec2, speed: f32) -> Self {
        Self {
            direction: Vec3::new(direction.x, direction.y, 0.0),
            speed,
        }
    }
}

#[derive(Bundle, Debug)]
pub struct MovingObjectBundle {
    pub velocity: Velocity,
}

fn update_position(mut movable_object_q: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for (movable_object_velocity, mut movable_object_transform) in movable_object_q.iter_mut() {
        movable_object_transform.translation +=
            movable_object_velocity.direction.normalize_or_zero()
                * f32::min(
                    movable_object_velocity.speed,
                    movable_object_velocity.direction.length() * movable_object_velocity.speed,
                )
                * time.delta_seconds();
    }
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_position);
    }
}
