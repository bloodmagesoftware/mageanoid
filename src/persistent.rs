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
use bevy::audio::Volume;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::state::*;

#[derive(Resource, Serialize, Deserialize, Default, Debug)]
pub struct Score {
    pub current_score: u32,
    pub high_score: u32,
}

impl Score {
    pub fn reset(&mut self) {
        self.current_score = 0;
    }

    pub fn increase(&mut self, amount: u32) {
        self.current_score += amount;
        self.high_score = self.high_score.max(self.current_score);
    }
}

#[derive(Resource, Serialize, Deserialize, Debug)]
pub struct Mixer {
    pub master: f32,
}

impl Default for Mixer {
    fn default() -> Self {
        Self { master: 1.0 }
    }
}

impl Mixer {
    pub fn as_volume(&self) -> Volume {
        Volume::new(self.master)
    }

    pub fn as_volume_with_multiplier(&self, multiplier: f32) -> Volume {
        Volume::new(self.master * multiplier)
    }
}

#[cfg(feature = "storage")]
fn reset_score(mut score: ResMut<bevy_persistent::Persistent<Score>>) {
    score.reset();
}

#[cfg(not(feature = "storage"))]
fn reset_score(mut score: ResMut<Score>) {
    score.reset();
}

#[cfg(feature = "storage")]
fn persist(score: Res<bevy_persistent::Persistent<Score>>) {
    info!("persist");
    score.persist().expect("failed to persist score");
}

pub struct PersistentPlugin;

impl Plugin for PersistentPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "storage")]
        {
            if let Some(config_dir) = dirs::config_dir() {
                let score_resource = bevy_persistent::Persistent::<Score>::builder()
                    .name("score")
                    .format(bevy_persistent::StorageFormat::Bincode)
                    .path(config_dir.join("mageanoid").join("score"))
                    .default(Score::default())
                    .revertible(true)
                    .revert_to_default_on_deserialization_errors(true)
                    .build()
                    .expect("failed to initialize score resource");

                app.insert_resource(score_resource);

                let mixer_resource = bevy_persistent::Persistent::<Mixer>::builder()
                    .name("mixer")
                    .format(bevy_persistent::StorageFormat::Bincode)
                    .path(config_dir.join("mageanoid").join("mixer"))
                    .default(Mixer::default())
                    .revertible(true)
                    .revert_to_default_on_deserialization_errors(true)
                    .build()
                    .expect("failed to initialize mixer resource");

                app.insert_resource(mixer_resource);
            }

            app.add_systems(ON_ENTER_GAMEPLAY, reset_score)
                .add_systems(OnExit(AppState::InGame), persist);
        }

        #[cfg(not(feature = "storage"))]
        {
            app.insert_resource(Score::default());
            app.insert_resource(Mixer::default());

            app.add_systems(ON_ENTER_GAMEPLAY, reset_score);
        }
    }
}
