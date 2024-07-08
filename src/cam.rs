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
use bevy::render::camera::*;

fn add_camera(mut commands: Commands) {
    let mut my_2d_camera_bundle = Camera2dBundle::default();
    my_2d_camera_bundle.projection.scaling_mode = ScalingMode::FixedVertical(1024.0);
    my_2d_camera_bundle.transform = Transform::from_xyz(0.0, 0.0, 0.0);

    commands.spawn((my_2d_camera_bundle, IsDefaultUiCamera));
}

pub struct CamPlugin;

impl Plugin for CamPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_camera);
    }
}
