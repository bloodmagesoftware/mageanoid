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

use crate::ext::IntoVec3;
use crate::gameplay::anim::*;
use crate::gameplay::movement::*;
use crate::persistent::Mixer;
use crate::state::{AppState, ON_EXIT_GAMEPLAY};

#[derive(Component, Debug)]
pub struct Projectile;

#[derive(Bundle, Debug)]
pub struct ProjectileBundle {
    pub projectile: Projectile,
    pub animated_sprite: AnimatedSpriteBundle,
    pub moving_object: MovingObjectBundle,
}

impl ProjectileBundle {
    pub fn spawn(
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
        position: Vec3,
        direction: Vec2,
        #[cfg(not(feature = "storage"))] mixer: Res<Mixer>,
        #[cfg(feature = "storage")] mixer: Res<bevy_persistent::Persistent<Mixer>>,
    ) {
        let texture = asset_server.load("sprites/projectile.png");
        let layout = TextureAtlasLayout::from_grid(UVec2::new(64, 64), 4, 1, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);

        let animation_indices = AnimationIndices::new(0, 3);

        let projectile = ProjectileBundle {
            projectile: Projectile,
            animated_sprite: AnimatedSpriteBundle {
                sprite: SpriteBundle {
                    texture,
                    transform: Transform {
                        translation: position + (direction * 32.0).xyz(),
                        rotation: Quat::IDENTITY,
                        scale: Vec3::ONE,
                    },
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
                velocity: Velocity::from_vec2(direction, 420.0),
            },
        };
        let attack_sound = AudioBundle {
            source: asset_server.load("sounds/56_Attack_03.wav"),
            settings: PlaybackSettings {
                mode: PlaybackMode::Despawn,
                volume: mixer.as_volume(),
                ..default()
            },
        };
        let thunder_sound = AudioBundle {
            source: asset_server.load("sounds/18_Thunder_02.wav"),
            settings: PlaybackSettings {
                mode: PlaybackMode::Despawn,
                volume: mixer.as_volume_with_multiplier(0.5),
                ..default()
            },
        };

        commands.spawn(projectile).with_children(|parent| {
            parent.spawn(attack_sound);
            parent.spawn(thunder_sound);
        });
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

fn despawn_projectile(mut commands: Commands, projectile_q: Query<Entity, With<Projectile>>) {
    for entity in projectile_q.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            projectile_out_of_bounds.run_if(in_state(AppState::InGame)),
        )
        .add_systems(ON_EXIT_GAMEPLAY, despawn_projectile);
    }
}
