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

use crate::gameplay::overlap::StaticObject;

const Z_INDEX: f32 = -100.0;

const MIN_POS: Vec2 = Vec2::new(-1640.0, 1693.0);
const MAX_POS: Vec2 = Vec2::new(1560.0, -1660.5);

pub fn pos_inside_level(pos: &Vec3) -> bool {
    pos.x >= MIN_POS.x && pos.x <= MAX_POS.x && pos.y >= MAX_POS.y && pos.y <= MIN_POS.y
}

fn spawn_level(mut commands: Commands, asset_server: Res<AssetServer>) {
    let ground = SpriteBundle {
        texture: asset_server.load("levels/Untitled/png/Level_0__Ground.png"),
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, Z_INDEX)),
        ..default()
    };

    let rock = SpriteBundle {
        texture: asset_server.load("levels/Untitled/png/Level_0__Rock.png"),
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
        ..default()
    };

    commands
        .spawn((StaticObject, ground))
        .with_children(|parent| {
            parent.spawn((StaticObject, rock));
        });
}

pub struct LdtkPlugin;

impl Plugin for LdtkPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_level);
    }
}
