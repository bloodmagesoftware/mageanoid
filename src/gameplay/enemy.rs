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
use bevy::prelude::*;
#[cfg(feature = "storage")]
use bevy_persistent::Persistent;
use bevy_prng::WyRand;
use bevy_rand::prelude::GlobalEntropy;
use rand_core::RngCore;

use crate::gameplay::anim::*;
use crate::gameplay::health::*;
use crate::gameplay::movement::*;
use crate::gameplay::player::*;
use crate::gameplay::projectile::*;
use crate::persistent::Score;
use crate::state::{AppState, ON_EXIT_GAMEPLAY};

const ENEMY_SPEED: f32 = 70.0;
const ENEMY_THRESHOLD: f32 = 64.0;
const ENEMY_MAX_COUNT: usize = 64;

pub struct EnemyPlugin;

#[derive(Debug)]
enum EnemyState {
    Hunting,
    ReadyBlade,
    SwingBlade,
}

#[derive(Debug)]
enum Face {
    Left,
    Right,
}

#[derive(Component, Debug)]
pub struct Enemy {
    animation_state: EnemyState,
    face: Face,
    sword_hit_timer: Timer,
}

impl Default for Enemy {
    fn default() -> Self {
        Self {
            animation_state: EnemyState::Hunting,
            face: Face::Left,
            sword_hit_timer: Timer::from_seconds(0.5, TimerMode::Repeating),
        }
    }
}

fn spawn_enemy(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    player_transform_q: Query<&Transform, With<Player>>,
    enemy_q: Query<&Enemy>,
    mut rng: ResMut<GlobalEntropy<WyRand>>,
) {
    let enemies_to_spawn = ENEMY_MAX_COUNT - enemy_q.iter().count();
    let player_transform = match player_transform_q.get_single() {
        Ok(player) => player,
        Err(_) => return,
    };

    for _ in 0..enemies_to_spawn {
        let texture = asset_server.load("sprites/skeleton.png");
        let layout = TextureAtlasLayout::from_grid(UVec2::new(64, 64), 4, 3, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);
        let animation_indices = AnimationIndices::new(0, 0);

        let radius = rng.next_u32() as f32 / u32::MAX as f32 * 2.0 * std::f32::consts::PI;
        let distance =
            rng.next_u32() as f32 / u32::MAX as f32 * ENEMY_SPEED * 15.0 + ENEMY_SPEED * 15.0;

        commands.spawn((
            Enemy::default(),
            Health::new(1.0),
            SpriteBundle {
                texture,
                transform: Transform {
                    translation: Vec3::new(
                        player_transform.translation.x + distance * radius.cos(),
                        player_transform.translation.y + distance * radius.sin(),
                        0.0,
                    ),
                    scale: Vec3::splat(1.0),
                    ..default()
                },
                ..default()
            },
            TextureAtlas {
                layout: texture_atlas_layout,
                index: animation_indices.first,
            },
            animation_indices,
            AnimationTimer(Timer::from_seconds(0.25, TimerMode::Repeating)),
            MovingObjectBundle {
                velocity: Velocity::from_vec3(Vec3::new(0.0, 0.0, 0.0), ENEMY_SPEED),
            },
        ));
    }
}

fn enemy_attack(
    mut enemy_q: Query<(&mut Enemy, &GlobalTransform)>,
    mut player_q: Query<(&Transform, &mut Health), With<Player>>,
    time: Res<Time>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for (mut enemy, enemy_transform) in enemy_q.iter_mut() {
        enemy.sword_hit_timer.tick(time.delta());
        if !enemy.sword_hit_timer.finished() {
            continue;
        }
        for (player_transform, mut player_health) in player_q.iter_mut() {
            let direction = player_transform.translation - enemy_transform.translation();
            if direction.length() <= ENEMY_THRESHOLD {
                enemy.animation_state = match enemy.animation_state {
                    EnemyState::Hunting => EnemyState::ReadyBlade,
                    EnemyState::SwingBlade => EnemyState::ReadyBlade,
                    EnemyState::ReadyBlade => {
                        enemy_attack_fx(&mut commands, &asset_server);
                        if player_health.damage(1.0) {
                            next_state.set(AppState::MainMenu);
                        }
                        EnemyState::SwingBlade
                    }
                };
            } else {
                enemy.animation_state = EnemyState::Hunting;
            }
        }
    }
}

fn projectile_hit_enemy(
    mut commands: Commands,
    mut enemy_q: Query<(Entity, &Transform, &mut Health), With<Enemy>>,
    projectile_q: Query<(Entity, &Transform), With<Projectile>>,
    asset_server: Res<AssetServer>,
    #[cfg(feature = "storage")] mut score: ResMut<Persistent<Score>>,
    #[cfg(not(feature = "storage"))] mut score: ResMut<Score>,
) {
    let mut fx = false;
    for (projectile_entity, projectile_transform) in projectile_q.iter() {
        for (enemy_entity, enemy_transform, mut enemy_health) in enemy_q.iter_mut() {
            if projectile_transform
                .translation
                .distance(enemy_transform.translation)
                < 64.0
            {
                if enemy_health.damage(1.0) {
                    commands.entity(enemy_entity).despawn_recursive();
                    score.increase(1);
                }
                if !fx {
                    fx = true;
                    enemy_hit_fx(&mut commands, &asset_server);
                }
                commands.entity(projectile_entity).despawn_recursive();
            }
        }
    }
}

fn enemy_hit_fx(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands.spawn(AudioBundle {
        source: asset_server.load("sounds/69_Enemy_death_01.wav"),
        settings: PlaybackSettings {
            mode: PlaybackMode::Despawn,
            ..default()
        },
    });
}

fn enemy_attack_fx(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands.spawn(AudioBundle {
        source: asset_server.load("sounds/56_Attack_03.wav"),
        settings: PlaybackSettings {
            mode: PlaybackMode::Despawn,
            ..default()
        },
    });
}

fn update_position(
    mut enemy_q: Query<(&Transform, &mut Velocity), With<Enemy>>,
    player_transform_q: Query<&Transform, With<Player>>,
) {
    if let Ok(player_transform) = player_transform_q.get_single() {
        for (enemy_transform, mut enemy_vel) in enemy_q.iter_mut() {
            let direction = player_transform.translation - enemy_transform.translation;
            if direction.length() >= ENEMY_THRESHOLD {
                enemy_vel.direction = direction.normalize();
            } else {
                enemy_vel.direction = Vec3::ZERO;
            }
        }
    }
}

fn update_animation(mut enemy_q: Query<(&mut Enemy, &Velocity, &mut AnimationIndices)>) {
    for (mut enemy, velocity, mut animation_indices) in enemy_q.iter_mut() {
        if velocity.direction.x < 0.0 {
            enemy.face = Face::Left;
        } else if velocity.direction.x > 0.0 {
            enemy.face = Face::Right;
        }

        animation_indices.first = match enemy.animation_state {
            EnemyState::Hunting => 0,
            EnemyState::ReadyBlade => 4,
            EnemyState::SwingBlade => 8,
        } + match enemy.face {
            Face::Left => 0,
            Face::Right => 2,
        };

        animation_indices.last = if velocity.direction.length() > 0.0 {
            animation_indices.first + 1
        } else {
            animation_indices.first
        };
    }
}

fn despawn_enemy(mut commands: Commands, enemy_q: Query<Entity, With<Enemy>>) {
    for enemy_entity in enemy_q.iter() {
        commands.entity(enemy_entity).despawn_recursive();
    }
}

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_enemy,
                update_position,
                enemy_attack,
                update_animation,
                projectile_hit_enemy,
            )
                .run_if(in_state(AppState::InGame)),
        )
        .add_systems(ON_EXIT_GAMEPLAY, despawn_enemy);
    }
}
