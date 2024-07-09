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

fn setup(mut commands: Commands) {
    #[cfg(feature = "storage")]
    {
        if let Some(config_dir) = dirs::config_dir() {
            let resource = bevy_persistent::Persistent::<Score>::builder()
                .name("score")
                .format(bevy_persistent::StorageFormat::Bincode)
                .path(config_dir.join("mageanoid").join("score"))
                .default(Score::default())
                .revertible(true)
                .revert_to_default_on_deserialization_errors(true)
                .build()
                .expect("failed to initialize score resource");

            commands.insert_resource(resource);
        }
    }
    #[cfg(not(feature = "storage"))]
    {
        commands.insert_resource(Score::default());
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
    score.persist().expect("failed to persist score");
}

pub struct PersistentPlugin;

impl Plugin for PersistentPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "storage")]
        {
            app.add_systems(PreStartup, setup)
                .add_systems(ON_ENTER_GAMEPLAY, reset_score)
                .add_systems(OnExit(AppState::InGame), persist);
        }

        #[cfg(not(feature = "storage"))]
        {
            app.add_systems(PreStartup, setup)
                .add_systems(ON_ENTER_GAMEPLAY, reset_score);
        }
    }
}
