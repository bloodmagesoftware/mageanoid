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

use bevy::audio::PlaybackMode;
use bevy::input::mouse::MouseButtonInput;
use bevy::prelude::*;

use crate::gameplay::anim::*;
use crate::gameplay::movement::*;
use crate::gameplay::projectile::*;
use crate::state::{AppState, ON_ENTER_GAMEPLAY, ON_EXIT_GAMEPLAY};

const PLAYER_MOVE_SPEED: f32 = 175.0;

pub struct PlayerPlugin;

#[derive(Component, Debug)]
pub struct Player {
    pub projectile_spawn_timer: Timer,
    pub walk_sound_timer: Timer,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            projectile_spawn_timer: Timer::from_seconds(0.2, TimerMode::Once),
            walk_sound_timer: Timer::from_seconds(0.4, TimerMode::Repeating),
        }
    }
}

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
        Player::default(),
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

#[allow(clippy::too_many_arguments)]
fn player_projectile(
    mut player_q: Query<(&mut Player, &GlobalTransform)>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    time: Res<Time>,

    // cursor click
    mut mousebtn_evr: EventReader<MouseButtonInput>,
    window_q: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,

    // gamepad
    gamepads: Res<Gamepads>,
    axes: Res<Axis<GamepadAxis>>,
) {
    for (mut player, player_transform) in player_q.iter_mut() {
        player.projectile_spawn_timer.tick(time.delta());
        if !player.projectile_spawn_timer.finished() {
            return;
        }

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
                    spell_sound_fx(&mut commands, &asset_server);

                    player.projectile_spawn_timer.reset();
                    return;
                }
            }
        }

        if let Some(gamepad) = gamepads.iter().next() {
            let x = axes
                .get(GamepadAxis::new(gamepad, GamepadAxisType::RightStickX))
                .unwrap_or(0.0);
            let y = axes
                .get(GamepadAxis::new(gamepad, GamepadAxisType::RightStickY))
                .unwrap_or(0.0);

            let direction = Vec2::new(x, y);

            if direction.length() > 0.25 {
                commands.spawn(ProjectileBundle::new(
                    &asset_server,
                    &mut texture_atlas_layouts,
                    player_transform.translation(),
                    direction.normalize(),
                ));
                spell_sound_fx(&mut commands, &asset_server);

                player.projectile_spawn_timer.reset();
                return;
            }
        }
    }
}

fn spell_sound_fx(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands.spawn(AudioBundle {
        source: asset_server.load("sounds/56_Attack_03.wav"),
        settings: PlaybackSettings {
            mode: PlaybackMode::Despawn,
            ..default()
        },
    });
    commands.spawn(AudioBundle {
        source: asset_server.load("sounds/18_Thunder_02.wav"),
        settings: PlaybackSettings {
            mode: PlaybackMode::Despawn,
            ..default()
        },
    });
}

fn player_step_sound_fx(
    mut player_q: Query<(&mut Player, &Velocity)>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
) {
    for (mut player, player_velocity) in player_q.iter_mut() {
        player.walk_sound_timer.tick(time.delta());
        if player_velocity.direction.length() <= 0.0 {
            continue;
        }

        if !player.walk_sound_timer.just_finished() {
            continue;
        }

        commands.spawn(AudioBundle {
            source: asset_server.load("sounds/03_Step_grass_03.wav"),
            settings: PlaybackSettings {
                mode: PlaybackMode::Despawn,
                ..default()
            },
        });
    }
}

fn player_movement(
    mut player_q: Query<(&mut Velocity, &mut AnimationIndices), With<Player>>,
    keys: Res<ButtonInput<KeyCode>>,
    gamepads: Res<Gamepads>,
    axes: Res<Axis<GamepadAxis>>,
    buttons: Res<ButtonInput<GamepadButton>>,
) {
    for (mut player_velocity, mut player_indices) in player_q.iter_mut() {
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

        // keyboard sprint
        if keys.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]) {
            player_velocity.speed = PLAYER_MOVE_SPEED * 2.0;
        } else {
            player_velocity.speed = PLAYER_MOVE_SPEED;
        }

        if let Some(gamepad) = gamepads.iter().next() {
            // left stick x
            if let Some(left_stick_x) =
                axes.get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickX))
            {
                if left_stick_x.abs() > 0.1 {
                    player_velocity.direction.x += left_stick_x;
                }
            }

            // left stick y
            if let Some(left_stick_y) =
                axes.get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickY))
            {
                if left_stick_y.abs() > 0.1 {
                    player_velocity.direction.y += left_stick_y;
                }
            }

            // sprint with face button bottom
            let sprint_button = GamepadButton {
                gamepad,
                button_type: GamepadButtonType::South,
            };
            if buttons.pressed(sprint_button) {
                player_velocity.speed = PLAYER_MOVE_SPEED * 2.0;
            }
        }

        // animation face direction
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

fn despawn_player(mut commands: Commands, query: Query<Entity, With<Player>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(ON_ENTER_GAMEPLAY, spawn_player);
        app.add_systems(
            Update,
            (player_movement, player_step_sound_fx, player_projectile)
                .run_if(in_state(AppState::InGame)),
        )
        .add_systems(ON_EXIT_GAMEPLAY, despawn_player);
    }
}
