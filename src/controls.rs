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
use std::ops::Deref;

use bevy::prelude::*;

#[derive(Resource, Component, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum ControlType {
    #[default]
    Keyboard,
    Gamepad,
}

fn gamepad(mut control: ResMut<ControlType>, gamepads: Res<Gamepads>) {
    if gamepads.iter().next().is_some() {
        *control = ControlType::Gamepad;
    } else {
        *control = ControlType::Keyboard;
    }
}

fn only_when_control_type(control: Res<ControlType>, mut query: Query<(&ControlType, &mut Style)>) {
    for (only_when_control_type, mut style) in query.iter_mut() {
        if only_when_control_type.eq(control.deref()) {
            style.display = Display::Block;
        } else {
            style.display = Display::None;
        }
    }
}

pub struct ControlPlugin;

impl Plugin for ControlPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ControlType::default())
            .add_systems(Update, gamepad)
            .add_systems(Update, only_when_control_type);
    }
}
