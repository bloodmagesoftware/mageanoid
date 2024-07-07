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

use crate::anim::*;
use crate::movement::*;

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
            velocity: Velocity::new(Vec3::new(0.0, 0.0, 0.0), PLAYER_MOVE_SPEED),
        },
    ));
}

fn player_input(
    mut player_entity: Query<(&Player, &mut Velocity, &mut AnimationIndices)>,
    keys: Res<ButtonInput<KeyCode>>,
    gamepads: Res<Gamepads>,
    axes: Res<Axis<GamepadAxis>>,
) {
    for (_, mut velocity, mut indices) in &mut player_entity.iter_mut() {
        // keyboard x
        if keys.pressed(KeyCode::ArrowLeft) || keys.pressed(KeyCode::KeyA) {
            velocity.direction.x = -1.0;
        } else if keys.pressed(KeyCode::ArrowRight) || keys.pressed(KeyCode::KeyD) {
            velocity.direction.x = 1.0;
        } else {
            velocity.direction.x = 0.0;
        }

        // keyboard y
        if keys.pressed(KeyCode::ArrowUp) || keys.pressed(KeyCode::KeyW) {
            velocity.direction.y = 1.0;
        } else if keys.pressed(KeyCode::ArrowDown) || keys.pressed(KeyCode::KeyS) {
            velocity.direction.y = -1.0;
        } else {
            velocity.direction.y = 0.0;
        }

        // gamepad
        for gamepad in gamepads.iter() {
            if let Some(left_stick_x) =
                axes.get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickX))
            {
                if left_stick_x.abs() > 0.1 {
                    velocity.direction.x += left_stick_x;
                }
            }

            if let Some(left_stick_y) =
                axes.get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickY))
            {
                if left_stick_y.abs() > 0.1 {
                    velocity.direction.y += left_stick_y;
                }
            }
        }

        // face
        if velocity.direction.x < 0.0 {
            indices.first = 0;
            indices.last = 1;
        } else if velocity.direction.x > 0.0 {
            indices.first = 2;
            indices.last = 3;
        } else {
            indices.last = indices.first;
        }
    }
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, player_input);
    }
}
