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
use bevy_prng::WyRand;
use bevy_rand::prelude::GlobalEntropy;
use rand_core::RngCore;

use crate::anim::{AnimationIndices, AnimationTimer};
use crate::health::Health;
use crate::movement::*;
use crate::player::Player;
use crate::projectile::Projectile;

const ENEMY_SPEED: f32 = 40.0;
const ENEMY_THRESHOLD: f32 = ENEMY_SPEED * 2.0;
const ENEMY_MAX_COUNT: usize = 10;

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
            rng.next_u32() as f32 / u32::MAX as f32 * ENEMY_SPEED * 10.0 + ENEMY_SPEED * 17.5;

        commands.spawn((
            Enemy::default(),
            Health::new(1),
            SpriteBundle {
                texture,
                transform: Transform::from_scale(Vec3::splat(2.0)).with_translation(Vec3::new(
                    player_transform.translation.x + distance * radius.cos(),
                    player_transform.translation.y + distance * radius.sin(),
                    0.0,
                )),
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
    player_q: Query<&Transform, With<Player>>,
    time: Res<Time>,
) {
    for (mut enemy, enemy_transform) in enemy_q.iter_mut() {
        enemy.sword_hit_timer.tick(time.delta());
        if !enemy.sword_hit_timer.finished() {
            continue;
        }
        for player_transform in player_q.iter() {
            let direction = player_transform.translation - enemy_transform.translation();
            if direction.length() <= ENEMY_THRESHOLD {
                enemy.animation_state = match enemy.animation_state {
                    EnemyState::Hunting => EnemyState::ReadyBlade,
                    EnemyState::SwingBlade => EnemyState::ReadyBlade,
                    EnemyState::ReadyBlade => EnemyState::SwingBlade,
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
) {
    for (projectile_entity, projectile_transform) in &mut projectile_q.iter() {
        for (enemy_entity, enemy_transform, mut enemy_health) in &mut enemy_q.iter_mut() {
            if projectile_transform
                .translation
                .distance(enemy_transform.translation)
                < 64.0
            {
                if enemy_health.damage(1) {
                    commands.entity(enemy_entity).despawn();
                }
                commands.entity(projectile_entity).despawn();
            }
        }
    }
}

fn update_position(
    mut enemy_q: Query<(&Transform, &mut Velocity), With<Enemy>>,
    player_transform_q: Query<&Transform, With<Player>>,
) {
    if let Ok(player_transform) = player_transform_q.get_single() {
        for (enemy_transform, mut enemy_vel) in &mut enemy_q.iter_mut() {
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

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_enemy)
            .add_systems(Update, update_position)
            .add_systems(Update, enemy_attack)
            .add_systems(Update, update_animation)
            .add_systems(Update, projectile_hit_enemy);
    }
}
