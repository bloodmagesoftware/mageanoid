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

use std::cmp;

use bevy::prelude::*;

use crate::movement::*;

const PLAYER_MOVE_SPEED: f32 = 4.0;


pub struct PlayerPlugin;


#[derive(Component, Debug)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas)>,
) {
    for (indices, mut timer, mut atlas) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            atlas.index = if atlas.index == indices.last {
                indices.first
            } else {
                cmp::min(atlas.index + 1, indices.last)
            };
        }
    }
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("sprites/mage_walk.png");
    let layout = TextureAtlasLayout::from_grid(Vec2::new(64.0, 64.0), 4, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let animation_indices = AnimationIndices { first: 0, last: 0 };

    commands.spawn((
        SpriteSheetBundle {
            texture,
            atlas: TextureAtlas {
                layout: texture_atlas_layout,
                index: animation_indices.first,
            },
            transform: Transform::from_scale(Vec3::splat(2.0)),
            ..default()
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.25, TimerMode::Repeating)),
        MovingObjectBundle {
            velocity: Velocity::new(Vec2::new(0.0, 0.0)),
        },
    ));
}

fn player_input(
    mut query: Query<(&mut Velocity, &mut AnimationIndices)>,
    keys: Res<ButtonInput<KeyCode>>,
    gamepads: Res<Gamepads>,
    axes: Res<Axis<GamepadAxis>>,
) {
    for (mut velocity, mut indices) in &mut query.iter_mut() {
        // keyboard x
        if keys.pressed(KeyCode::ArrowLeft) {
            velocity.value.x = -PLAYER_MOVE_SPEED;
        } else if keys.pressed(KeyCode::ArrowRight) {
            velocity.value.x = PLAYER_MOVE_SPEED;
        } else {
            velocity.value.x = 0.0;
        }

        if keys.pressed(KeyCode::ArrowUp) {
            velocity.value.y = PLAYER_MOVE_SPEED;
        } else if keys.pressed(KeyCode::ArrowDown) {
            velocity.value.y = -PLAYER_MOVE_SPEED;
        } else {
            velocity.value.y = 0.0;
        }

        // gamepad
        for gamepad in gamepads.iter() {
            if let Some(left_stick_x) = axes.get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickX)) {
                if left_stick_x.abs() > 0.1 {
                    velocity.value.x = left_stick_x * PLAYER_MOVE_SPEED;
                } else {
                    velocity.value.x = 0.0;
                }
            }

            if let Some(left_stick_y) = axes.get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickY)) {
                if left_stick_y.abs() > 0.1 {
                    velocity.value.y = left_stick_y * PLAYER_MOVE_SPEED;
                } else {
                    velocity.value.y = 0.0;
                }
            }
        }


        // face
        if velocity.value.x < 0.0 {
            velocity.face = FaceDirection::Right;
            indices.first = 0;
            indices.last = 1;
        } else if velocity.value.x > 0.0 {
            velocity.face = FaceDirection::Left;
            indices.first = 2;
            indices.last = 3;
        } else {
            indices.last = indices.first;
        }
    }
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_player)
            .add_systems(Update, animate_sprite)
            .add_systems(Update, player_input);
    }
}
