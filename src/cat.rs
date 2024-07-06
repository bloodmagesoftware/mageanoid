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

use crate::anim::{AnimationIndices, AnimationTimer};
use crate::movement::*;
use crate::player::Player;

const CAT_SPEED: f32 = 75.0;
const CAT_THRESHOLD: f32 = 100.0;

pub struct CatPlugin;

#[derive(Component, Debug)]
pub struct Cat;

fn spawn_cat(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("sprites/cat_walk.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::new(64, 64), 4, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let animation_indices = AnimationIndices::new(0, 0);

    commands.spawn((
        Cat,
        SpriteSheetBundle {
            texture,
            atlas: TextureAtlas {
                layout: texture_atlas_layout,
                index: animation_indices.first,
            },
            transform: Transform::from_scale(Vec3::splat(2.0)).with_translation(Vec3::new(
                CAT_THRESHOLD,
                CAT_THRESHOLD,
                0.0,
            )),
            ..default()
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.25, TimerMode::Repeating)),
        MovingObjectBundle {
            velocity: Velocity::new(Vec3::new(0.0, 0.0, 0.0), CAT_SPEED),
        },
    ));
}

fn update_position(
    mut cats: Query<(&Cat, &Transform, &mut Velocity)>,
    players: Query<(&Player, &Transform)>,
) {
    if let Some((_, player_transform)) = players.iter().next() {
        for (_, cat_transform, mut cat_vel) in &mut cats.iter_mut() {
            // calc direction from cat to player
            let direction = player_transform.translation - cat_transform.translation;
            if direction.length() > CAT_THRESHOLD {
                cat_vel.direction = direction.normalize();
            } else {
                cat_vel.direction = Vec3::ZERO;
            }
        }
    };
}

fn update_animation(mut cats: Query<(&Cat, &Velocity, &mut AnimationIndices)>) {
    for (_, velocity, mut indices) in &mut cats {
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

impl Plugin for CatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_cat)
            .add_systems(Update, update_position)
            .add_systems(Update, update_animation);
    }
}
