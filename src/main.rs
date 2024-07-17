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
use bevy_prng::WyRand;
use bevy_rand::prelude::EntropyPlugin;

mod cam;
mod controls;
mod ext;
mod gameplay;
mod ldtk;
mod mainmenu;
mod music;
mod persistent;
mod state;
mod style;
mod volume;

fn main() {
    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Mageanoid".into(),
                    name: Some("Mageanoid".into()),
                    present_mode: bevy_window::PresentMode::AutoVsync,
                    mode: bevy_window::WindowMode::BorderlessFullscreen,
                    fit_canvas_to_parent: true,
                    prevent_default_event_handling: false,
                    window_theme: Some(bevy_window::WindowTheme::Dark),
                    ..default()
                }),
                ..default()
            }),
    )
    .add_plugins(EntropyPlugin::<WyRand>::default())
    .add_plugins(cam::CamPlugin)
    .add_plugins(controls::ControlPlugin)
    .add_plugins(gameplay::GameplayPlugin)
    .add_plugins(ldtk::LdtkPlugin)
    .add_plugins(mainmenu::MainMenuPlugin)
    .add_plugins(music::MusicPlugin)
    .add_plugins(persistent::PersistentPlugin)
    .add_plugins(state::AppStatePlugin)
    .add_plugins(style::StylePlugin)
    .add_plugins(volume::VolumePlugin);

    app.run();
}
