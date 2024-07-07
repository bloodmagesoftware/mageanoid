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

use bevy::input::mouse::MouseButtonInput;
use bevy::prelude::*;

use crate::anim::*;
use crate::movement::*;
use crate::projectile::ProjectileBundle;

const PLAYER_MOVE_SPEED: f32 = 150.0;

pub struct PlayerPlugin;

#[derive(Component, Debug)]
pub struct Player;

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("sprites/mage_walk.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::new(64, 64), 4, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let animation_indices = AnimationIndices::new(0, 0);

    commands.spawn((
        Player,
        SpriteBundle {
            texture,
            transform: Transform::from_scale(Vec3::splat(2.0)),
            ..default()
        },
        TextureAtlas {
            layout: texture_atlas_layout,
            index: animation_indices.first,
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.25, TimerMode::Repeating)),
        MovingObjectBundle {
            velocity: Velocity::from_vec3(Vec3::new(0.0, 0.0, 0.0), PLAYER_MOVE_SPEED),
        },
    ));
}

fn player_projectile(
    player_transform_q: Query<&GlobalTransform, With<Player>>,
    mut mousebtn_evr: EventReader<MouseButtonInput>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,

    // cursor click
    window_q: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
) {
    for player_transform in &mut player_transform_q.iter() {
        for ev in mousebtn_evr.read() {
            if ev.state.is_pressed() && ev.button == MouseButton::Left {
                let window = match window_q.get_single() {
                    Ok(window) => window,
                    Err(_) => return,
                };

                let (camera, camera_transform) = match camera_q.get_single() {
                    Ok(cam) => cam,
                    Err(_) => return,
                };

                if let Some(world_position) = window
                    .cursor_position()
                    .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
                {
                    let direction =
                        (world_position - player_transform.translation().xy()).normalize();

                    commands.spawn(ProjectileBundle::new(
                        &asset_server,
                        &mut texture_atlas_layouts,
                        player_transform.translation(),
                        direction,
                    ));
                }
            }
        }
    }
}

fn player_movement(
    mut player_q: Query<(&mut Velocity, &mut AnimationIndices), With<Player>>,
    keys: Res<ButtonInput<KeyCode>>,
    gamepads: Res<Gamepads>,
    axes: Res<Axis<GamepadAxis>>,
) {
    for (mut player_velocity, mut player_indices) in &mut player_q.iter_mut() {
        // keyboard x
        if keys.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) {
            player_velocity.direction.x = -1.0;
        } else if keys.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) {
            player_velocity.direction.x = 1.0;
        } else {
            player_velocity.direction.x = 0.0;
        }

        // keyboard y
        if keys.any_pressed([KeyCode::ArrowUp, KeyCode::KeyW]) {
            player_velocity.direction.y = 1.0;
        } else if keys.any_pressed([KeyCode::ArrowDown, KeyCode::KeyS]) {
            player_velocity.direction.y = -1.0;
        } else {
            player_velocity.direction.y = 0.0;
        }

        // gamepad
        for gamepad in gamepads.iter() {
            if let Some(left_stick_x) =
                axes.get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickX))
            {
                if left_stick_x.abs() > 0.1 {
                    player_velocity.direction.x += left_stick_x;
                }
            }

            if let Some(left_stick_y) =
                axes.get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickY))
            {
                if left_stick_y.abs() > 0.1 {
                    player_velocity.direction.y += left_stick_y;
                }
            }
        }

        // face
        if player_velocity.direction.x < 0.0 {
            player_indices.first = 0;
            player_indices.last = 1;
        } else if player_velocity.direction.x > 0.0 {
            player_indices.first = 2;
            player_indices.last = 3;
        } else {
            player_indices.last = player_indices.first;
        }
    }
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, player_movement)
            .add_systems(Update, player_projectile);
    }
}
