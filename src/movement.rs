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

use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Moving {
    pub velocity: Vec2,
}

fn update_position(mut query: Query<(&Moving, &mut Transform)>, time: Res<Time>) {
    for (moving, mut transform) in query.iter_mut() {
        transform.translation +=
            Vec3::new(moving.velocity.x, moving.velocity.y, 0.0) * time.delta_seconds();
    }
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(Update, update_position);
    }
}
