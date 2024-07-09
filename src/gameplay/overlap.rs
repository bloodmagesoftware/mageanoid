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

use crate::state::AppState;

#[derive(Component, Debug)]
pub struct StaticObject;

fn overlap(mut transform_q: Query<&mut Transform, Without<StaticObject>>) {
    for mut transform in transform_q.iter_mut() {
        transform.translation.z = -transform.translation.y / 1000.0;
    }
}

pub struct OverlapPlugin;

impl Plugin for OverlapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, overlap.run_if(in_state(AppState::InGame)));
    }
}
