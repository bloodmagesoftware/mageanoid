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

use bevy::input::gamepad::*;
use bevy::prelude::*;

use crate::player::PlayerPlugin;

fn gamepad_system(
    mut gamepads: ResMut<Gamepads>,
    axes: Res<Axis<GamepadAxis>>,
    button_inputs: Res<ButtonInput<GamepadButton>>,
    button_axes: Res<Axis<GamepadButton>>,
) {
    for gamepad in gamepads.iter() {
        let left_stick_x = axes.get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickX));
        let left_stick_y = axes.get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickY));
        info!("{:?} left stick: ({:?}, {:?})", gamepad, left_stick_x, left_stick_y);
    }
}

pub struct GamepadPlugin;

impl Plugin for GamepadPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, gamepad_system);
    }
}
