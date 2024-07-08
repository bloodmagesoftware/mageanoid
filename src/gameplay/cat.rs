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

use crate::gameplay::anim::*;
use crate::gameplay::movement::*;
use crate::gameplay::player::*;
use crate::state::{AppState, ON_ENTER_GAMEPLAY, ON_EXIT_GAMEPLAY};

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
        AnimatedSpriteBundle {
            sprite: SpriteBundle {
                texture,
                transform: Transform::from_scale(Vec3::splat(2.0)),
                ..default()
            },
            atlas: TextureAtlas {
                layout: texture_atlas_layout,
                index: animation_indices.first,
            },
            indices: animation_indices,
            timer: AnimationTimer(Timer::from_seconds(0.25, TimerMode::Repeating)),
        },
        MovingObjectBundle {
            velocity: Velocity::from_vec3(Vec3::new(0.0, 0.0, 0.0), CAT_SPEED),
        },
    ));
}

fn update_position(
    mut cat_q: Query<(&Transform, &mut Velocity), With<Cat>>,
    player_transform_q: Query<&Transform, With<Player>>,
) {
    let player_transform = match player_transform_q.get_single() {
        Ok(player_transform) => player_transform,
        Err(_) => return,
    };

    for (cat_transform, mut cat_velocity) in cat_q.iter_mut() {
        // calc direction from cat to player
        let direction = player_transform.translation - cat_transform.translation;
        if direction.length() > CAT_THRESHOLD {
            cat_velocity.direction = direction.normalize();
        } else {
            cat_velocity.direction = Vec3::ZERO;
        }
    }
}

fn update_animation(mut cats: Query<(&Velocity, &mut AnimationIndices), With<Cat>>) {
    for (velocity, mut indices) in cats.iter_mut() {
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

fn despawn_cat(mut commands: Commands, cat_q: Query<Entity, With<Cat>>) {
    for cat_entity in cat_q.iter() {
        commands.entity(cat_entity).despawn_recursive();
    }
}

impl Plugin for CatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(ON_ENTER_GAMEPLAY, spawn_cat)
            .add_systems(
                Update,
                (update_position, update_animation).run_if(in_state(AppState::InGame)),
            )
            .add_systems(ON_EXIT_GAMEPLAY, despawn_cat);
    }
}
