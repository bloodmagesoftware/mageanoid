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
use bevy_persistent::prelude::*;
use serde::{Deserialize, Serialize};

use crate::state::{AppState, ON_ENTER_GAMEPLAY};

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
    let config_dir = dirs::config_dir().unwrap().join("mageanoid");
    let resource = Persistent::<Score>::builder()
        .name("score")
        .format(StorageFormat::Toml)
        .path(config_dir.join("score.toml"))
        .default(Score::default())
        .build()
        .expect("failed to initialize score resource");

    commands.insert_resource(resource);

    info!("Score resource initialized");
}

fn reset_score(mut score: ResMut<Persistent<Score>>) {
    score.reset();
}

fn persist(score: Res<Persistent<Score>>) {
    score.persist().expect("failed to persist score");
}

pub struct PersistentPlugin;

impl Plugin for PersistentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(ON_ENTER_GAMEPLAY, reset_score)
            .add_systems(OnExit(AppState::InGame), persist);
    }
}
