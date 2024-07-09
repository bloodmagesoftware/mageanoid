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

#[derive(Component, Debug)]
pub struct Health {
    pub health: f32,
    pub max_health: f32,
}

impl Health {
    pub fn new(max_health: f32) -> Self {
        Self {
            health: max_health,
            max_health,
        }
    }

    /**
     * Returns true if the entity is dead
     */
    pub fn damage(&mut self, damage: f32) -> bool {
        self.health = (self.health - damage).max(0.0);
        self.health == 0.0
    }

    /**
     * Returns the health percentage [0.0, 100.0]
     */
    pub fn health_percentage(&self) -> f32 {
        (self.health / self.max_health) * 100.0
    }
}

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, _app: &mut App) {}
}
