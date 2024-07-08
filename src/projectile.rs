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

use crate::anim::{AnimatedSpriteBundle, AnimationIndices, AnimationTimer};
use crate::movement::{MovingObjectBundle, Velocity};

#[derive(Component, Debug)]
pub struct Projectile;

#[derive(Bundle, Debug)]
pub struct ProjectileBundle {
    pub projectile: Projectile,
    pub animated_sprite: AnimatedSpriteBundle,
    pub moving_object: MovingObjectBundle,
}

impl ProjectileBundle {
    pub fn new(
        asset_server: &Res<AssetServer>,
        texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
        position: Vec3,
        direction: Vec2,
    ) -> Self {
        let texture = asset_server.load("sprites/projectile.png");
        let layout = TextureAtlasLayout::from_grid(UVec2::new(64, 64), 4, 1, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);

        let animation_indices = AnimationIndices::new(0, 3);

        ProjectileBundle {
            projectile: Projectile,
            animated_sprite: AnimatedSpriteBundle {
                sprite: SpriteBundle {
                    texture,
                    transform: Transform::from_scale(Vec3::splat(2.0)).with_translation(position),
                    ..default()
                },
                atlas: TextureAtlas {
                    layout: texture_atlas_layout,
                    index: animation_indices.first,
                },
                indices: animation_indices,
                timer: AnimationTimer(Timer::from_seconds(0.125, TimerMode::Repeating)),
            },
            moving_object: MovingObjectBundle {
                velocity: Velocity::from_vec2(direction, 512.0),
            },
        }
    }
}

fn projectile_out_of_bounds(
    projectile_q: Query<(Entity, &ViewVisibility), With<Projectile>>,
    mut commands: Commands,
) {
    for (projectile_entity, view_visibility) in projectile_q.iter() {
        if !view_visibility.get() {
            commands.entity(projectile_entity).despawn();
        }
    }
}

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, _app: &mut App) {
        _app.add_systems(Update, projectile_out_of_bounds);
    }
}
