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
use bevy_prng::WyRand;
use bevy_rand::prelude::GlobalEntropy;
use rand_core::RngCore;

use crate::ext::{IntoVec3, RandomAround};
use crate::gameplay::anim::*;
use crate::gameplay::enemy_die::EnemyDied;
use crate::gameplay::health::*;
use crate::gameplay::movement::*;
use crate::gameplay::player::*;
use crate::gameplay::projectile::*;
use crate::persistent::{Mixer, Score};
use crate::state::{AppState, ON_ENTER_GAMEPLAY, ON_EXIT_GAMEPLAY};

const ENEMY_THRESHOLD: f32 = 64.0;

#[derive(Debug)]
enum EnemyState {
    Hunting,
    ReadyBlade,
    SwingBlade,
}

#[derive(Debug)]
enum EnemyTarget {
    Player,
    PlayerFuture,
    Location(Vec2),
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
    target_switch_timer: Timer,
    target: EnemyTarget,
}

impl Default for Enemy {
    fn default() -> Self {
        Self {
            animation_state: EnemyState::Hunting,
            face: Face::Left,
            sword_hit_timer: Timer::from_seconds(0.3, TimerMode::Repeating),
            target_switch_timer: Timer::from_seconds(6.0, TimerMode::Repeating),
            target: EnemyTarget::Player,
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
    difficulty: Res<EnemyDifficulty>,
) {
    let enemies_to_spawn = difficulty.get_enemy_max_count() - enemy_q.iter().count();
    let player_transform = match player_transform_q.get_single() {
        Ok(player) => player,
        Err(_) => return,
    };

    for _ in 0..enemies_to_spawn {
        let texture = asset_server.load("sprites/skeleton.png");
        let layout = TextureAtlasLayout::from_grid(UVec2::new(64, 64), 4, 3, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);
        let animation_indices = AnimationIndices::new(0, 0);

        commands.spawn((
            Enemy::default(),
            Health::new(1.0),
            SpriteBundle {
                texture,
                transform: Transform {
                    translation: player_transform.translation.random_around(
                        &mut rng,
                        difficulty.get_enemy_speed() * 15.0,
                        difficulty.get_enemy_speed() * 30.0,
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
                velocity: Velocity::from_vec3(
                    Vec3::new(0.0, 0.0, 0.0),
                    difficulty.get_enemy_speed(),
                ),
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
    #[cfg(not(feature = "storage"))] mixer: Res<Mixer>,
    #[cfg(feature = "storage")] mixer: Res<bevy_persistent::Persistent<Mixer>>,
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
                        enemy_attack_fx(&mut commands, &asset_server, mixer);
                        if player_health.damage(1.0) {
                            next_state.set(AppState::Death);
                        }
                        EnemyState::SwingBlade
                    }
                };
                return;
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
    mut evw_enemy_died: EventWriter<EnemyDied>,
    #[cfg(feature = "storage")] mut score: ResMut<bevy_persistent::Persistent<Score>>,
    #[cfg(not(feature = "storage"))] mut score: ResMut<Score>,
    #[cfg(not(feature = "storage"))] mixer: Res<Mixer>,
    #[cfg(feature = "storage")] mixer: Res<bevy_persistent::Persistent<Mixer>>,
) {
    for (projectile_entity, projectile_transform) in projectile_q.iter() {
        for (enemy_entity, enemy_transform, mut enemy_health) in enemy_q.iter_mut() {
            if projectile_transform
                .translation
                .distance(enemy_transform.translation)
                <= 64.0
            {
                if enemy_health.damage(1.0) {
                    evw_enemy_died.send(EnemyDied {
                        pos: enemy_transform.translation,
                    });
                    commands.entity(enemy_entity).despawn_recursive();
                    score.increase(1);
                }
                enemy_hit_fx(&mut commands, &asset_server, mixer);
                commands.entity(projectile_entity).despawn_recursive();
                return;
            }
        }
    }
}

fn enemy_hit_fx(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    #[cfg(not(feature = "storage"))] mixer: Res<Mixer>,
    #[cfg(feature = "storage")] mixer: Res<bevy_persistent::Persistent<Mixer>>,
) {
    commands.spawn(AudioBundle {
        source: asset_server.load("sounds/69_Enemy_death_01.wav"),
        settings: PlaybackSettings {
            mode: PlaybackMode::Despawn,
            volume: mixer.as_volume(),
            ..default()
        },
    });
}

fn enemy_attack_fx(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    #[cfg(not(feature = "storage"))] mixer: Res<Mixer>,
    #[cfg(feature = "storage")] mixer: Res<bevy_persistent::Persistent<Mixer>>,
) {
    commands.spawn(AudioBundle {
        source: asset_server.load("sounds/56_Attack_03.wav"),
        settings: PlaybackSettings {
            mode: PlaybackMode::Despawn,
            volume: mixer.as_volume(),
            ..default()
        },
    });
}

fn switch_target(
    mut enemy_q: Query<&mut Enemy>,
    time: Res<Time>,
    mut rng: ResMut<GlobalEntropy<WyRand>>,
    player_transform_q: Query<&GlobalTransform, With<Player>>,
) {
    let player_transform = match player_transform_q.get_single() {
        Ok(player_transform) => player_transform,
        Err(_) => return,
    };
    for mut enemy in enemy_q.iter_mut() {
        enemy.target_switch_timer.tick(time.delta());
        if enemy.target_switch_timer.just_finished() {
            let target = match rng.next_u32() % 3 {
                0 => EnemyTarget::Player,
                1 => EnemyTarget::PlayerFuture,
                _ => EnemyTarget::Location(
                    player_transform
                        .translation()
                        .random_around(&mut rng, 128.0, 512.0)
                        .xy(),
                ),
            };
            enemy.target = target;
        }
    }
}

fn update_position(
    mut enemy_q: Query<(&Enemy, &Transform, &mut Velocity), Without<Player>>,
    player_q: Query<(&Transform, &Velocity), With<Player>>,
) {
    if let Ok((player_transform, player_velocity)) = player_q.get_single() {
        for (enemy, enemy_transform, mut enemy_vel) in enemy_q.iter_mut() {
            let target_pos = match enemy.target {
                EnemyTarget::Player => player_transform.translation,
                EnemyTarget::PlayerFuture => {
                    player_transform.translation + player_velocity.direction * 128.0
                }
                EnemyTarget::Location(pos) => pos.xyz(),
            };
            let direction = target_pos - enemy_transform.translation;
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

fn increase_difficulty(mut difficulty: ResMut<EnemyDifficulty>, time: Res<Time>) {
    difficulty.enemy_speed = (difficulty.enemy_speed + time.delta_seconds() * 0.2).min(150.0);
    difficulty.enemy_max_count = (difficulty.enemy_max_count + time.delta_seconds()).min(100.0);
}

fn reset_difficulty(mut difficulty: ResMut<EnemyDifficulty>) {
    *difficulty = EnemyDifficulty::default();
}

fn despawn_enemy(mut commands: Commands, enemy_q: Query<Entity, With<Enemy>>) {
    for enemy_entity in enemy_q.iter() {
        commands.entity(enemy_entity).despawn_recursive();
    }
}

#[derive(Resource, Debug)]
pub struct EnemyDifficulty {
    enemy_speed: f32,
    enemy_max_count: f32,
}

impl Default for EnemyDifficulty {
    fn default() -> Self {
        Self {
            enemy_speed: 60.0,
            enemy_max_count: 32.0,
        }
    }
}

impl EnemyDifficulty {
    fn get_enemy_speed(&self) -> f32 {
        self.enemy_speed
    }

    fn get_enemy_max_count(&self) -> usize {
        self.enemy_max_count as usize
    }
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EnemyDifficulty::default())
            .add_systems(ON_ENTER_GAMEPLAY, reset_difficulty)
            .add_systems(
                Update,
                (
                    spawn_enemy,
                    switch_target,
                    update_position,
                    enemy_attack,
                    update_animation,
                    projectile_hit_enemy,
                    increase_difficulty,
                )
                    .run_if(in_state(AppState::InGame)),
            )
            .add_systems(ON_EXIT_GAMEPLAY, despawn_enemy);
    }
}
