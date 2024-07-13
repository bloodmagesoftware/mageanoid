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

use bevy::prelude::{Transform, Vec2, Vec3};
use rand_core::RngCore;

pub trait FRng {
    fn next_f32(&mut self) -> f32;
    fn next_f32_range(&mut self, min: f32, max: f32) -> f32 {
        min + (max - min) * self.next_f32()
    }
}

impl FRng for wyrand::WyRand {
    fn next_f32(&mut self) -> f32 {
        self.rand() as f32 / u32::MAX as f32
    }
    fn next_f32_range(&mut self, min: f32, max: f32) -> f32 {
        min + (max - min) * self.next_f32()
    }
}

impl FRng for bevy_prng::WyRand {
    fn next_f32(&mut self) -> f32 {
        self.next_u32() as f32 / u32::MAX as f32
    }
    fn next_f32_range(&mut self, min: f32, max: f32) -> f32 {
        min + (max - min) * self.next_f32()
    }
}

impl FRng for bevy::prelude::ResMut<'_, bevy_rand::prelude::GlobalEntropy<bevy_prng::WyRand>> {
    fn next_f32(&mut self) -> f32 {
        self.next_u32() as f32 / u32::MAX as f32
    }
    fn next_f32_range(&mut self, min: f32, max: f32) -> f32 {
        min + (max - min) * self.next_f32()
    }
}

pub trait IntoVec3 {
    fn xyz(self) -> Vec3;
}

impl IntoVec3 for Vec2 {
    fn xyz(self) -> Vec3 {
        Vec3::new(self.x, self.y, 0.0)
    }
}

pub trait RandomAround {
    fn random_around(self, rng: &mut impl FRng, min_radius: f32, max_radius: f32) -> Self;
}

impl RandomAround for Vec2 {
    fn random_around(self, rng: &mut impl FRng, min_radius: f32, max_radius: f32) -> Self {
        let radius = rng.next_f32() * 2.0 * std::f32::consts::PI;
        let distance = rng.next_f32_range(min_radius, max_radius);
        Vec2::new(
            self.x + distance * radius.cos(),
            self.y + distance * radius.sin(),
        )
    }
}

impl RandomAround for Vec3 {
    fn random_around(self, rng: &mut impl FRng, min_radius: f32, max_radius: f32) -> Self {
        let radius = rng.next_f32() * 2.0 * std::f32::consts::PI;
        let distance = rng.next_f32_range(min_radius, max_radius);
        Vec3::new(
            self.x + distance * radius.cos(),
            self.y + distance * radius.sin(),
            0.0,
        )
    }
}

impl RandomAround for Transform {
    fn random_around(self, rng: &mut impl FRng, min_radius: f32, max_radius: f32) -> Self {
        Transform {
            translation: self.translation.random_around(rng, min_radius, max_radius),
            ..self
        }
    }
}
