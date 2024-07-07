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
const ENEMY_THRESHOLD: f32 = ENEMY_SPEED;
const ENEMY_MAX_COUNT: usize = 10;

pub struct EnemyPlugin;

#[derive(Component, Debug)]
pub struct Enemy;

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
            rng.next_u32() as f32 / u32::MAX as f32 * ENEMY_SPEED * 20.0 + ENEMY_SPEED * 20.0;

        commands.spawn((
            Enemy,
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
            if direction.length() > ENEMY_THRESHOLD {
                enemy_vel.direction = direction.normalize();
            } else {
                enemy_vel.direction = Vec3::ZERO;
            }
        }
    }
}

fn update_animation(mut enemy_q: Query<(&Velocity, &mut AnimationIndices), With<Enemy>>) {
    for (velocity, mut indices) in &mut enemy_q {
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

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_enemy)
            .add_systems(Update, update_position)
            .add_systems(Update, update_animation)
            .add_systems(Update, projectile_hit_enemy);
    }
}
