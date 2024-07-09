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

use bevy::core_pipeline::tonemapping::DebandDither;
use bevy::prelude::*;
use bevy::render::camera::*;

use crate::gameplay::player::Player;
use crate::state::AppState;

#[derive(Component, Debug)]
struct CameraLag {
    pub lag: f32,
}

impl Default for CameraLag {
    fn default() -> Self {
        Self { lag: 0.5 }
    }
}

fn add_camera(mut commands: Commands) {
    let mut my_2d_camera_bundle = Camera2dBundle::default();
    my_2d_camera_bundle.projection.scaling_mode = ScalingMode::FixedVertical(750.0);
    my_2d_camera_bundle.transform = Transform::from_xyz(0.0, 0.0, 0.0);
    my_2d_camera_bundle.deband_dither = DebandDither::Enabled;

    commands.spawn((my_2d_camera_bundle, IsDefaultUiCamera, CameraLag::default()));
}

fn follow_player(
    mut camera_q: Query<(&mut Transform, &CameraLag), With<Camera>>,
    player_q: Query<&GlobalTransform, With<Player>>,
) {
    let player_transform = match player_q.get_single() {
        Ok(player_transform) => player_transform,
        Err(_) => {
            return;
        }
    };

    for (mut camera_transform, camera_lag) in camera_q.iter_mut() {
        camera_transform.translation = camera_transform.translation.lerp(
            Vec3::new(
                player_transform.translation().x,
                player_transform.translation().y,
                camera_transform.translation.z,
            ),
            camera_lag.lag,
        );
    }
}

pub struct CamPlugin;

impl Plugin for CamPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_camera)
            .add_systems(Update, follow_player.run_if(in_state(AppState::InGame)));
    }
}
