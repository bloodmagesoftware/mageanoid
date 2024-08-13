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

use crate::gameplay::anim::AnimationIndices;
use crate::state::AppState;
use bevy::prelude::*;

#[derive(Event)]
pub struct EnemyDied {
    pub pos: Vec3,
}

#[derive(Component, Debug)]
pub struct DeadEnemy {
    pub animation_timer: Timer,
    pub despawn_timer: Timer,
}

fn spawn_dead_enemy(
    mut commands: Commands,
    mut evr_enemy_died: EventReader<EnemyDied>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    for ev in evr_enemy_died.read() {
        let texture = asset_server.load("sprites/skeleton_die.png");
        let layout = TextureAtlasLayout::from_grid(UVec2::new(128, 128), 4, 1, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);
        let animation_indices = AnimationIndices::new(0, 3);

        commands.spawn((
            DeadEnemy {
                animation_timer: Timer::from_seconds(0.1, TimerMode::Repeating),
                despawn_timer: Timer::from_seconds(4.0, TimerMode::Once),
            },
            SpriteBundle {
                texture,
                transform: Transform {
                    translation: ev.pos,
                    scale: Vec3::splat(0.75),
                    ..default()
                },
                ..default()
            },
            TextureAtlas {
                layout: texture_atlas_layout,
                index: animation_indices.first,
            },
            animation_indices,
        ));
    }
}

fn update_animation(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut DeadEnemy, &mut TextureAtlas)>,
) {
    for (indices, mut dead_enemy, mut atlas) in query.iter_mut() {
        dead_enemy.animation_timer.tick(time.delta());
        if dead_enemy.animation_timer.just_finished() && atlas.index < indices.last {
            atlas.index += 1;
        }
    }
}

fn despawn_dead_enemy(
    mut commands: Commands,
    mut query: Query<(Entity, &mut DeadEnemy)>,
    time: Res<Time>,
) {
    for (entity, mut dead_enemy) in query.iter_mut() {
        dead_enemy.despawn_timer.tick(time.delta());
        if dead_enemy.despawn_timer.just_finished() {
            commands.entity(entity).despawn();
        }
    }
}

pub struct EnemyDiePlugin;

impl Plugin for EnemyDiePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EnemyDied>()
            .add_systems(
                PostUpdate,
                spawn_dead_enemy.run_if(in_state(AppState::InGame)),
            )
            .add_systems(
                Update,
                (update_animation, despawn_dead_enemy).run_if(in_state(AppState::InGame)),
            );
    }
}
