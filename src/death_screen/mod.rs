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

use crate::state::AppState;
use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct DeathScreen {
    pub remove_timer: Timer,
}

fn spawn_death_screen(mut commands: Commands) {
    commands
        .spawn((
            DeathScreen {
                remove_timer: Timer::from_seconds(3.0, TimerMode::Once),
            },
            NodeBundle {
                style: Style {
                    width: Val::Vw(100.0),
                    height: Val::Vh(100.0),
                    position_type: PositionType::Absolute,
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: BackgroundColor(Color::BLACK),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "You died!",
                TextStyle {
                    font_size: 64.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));
        });
}

fn remove_death_screen(
    mut commands: Commands,
    mut next_state: ResMut<NextState<AppState>>,
    time: Res<Time>,
    mut query: Query<(Entity, &mut DeathScreen)>,
) {
    for (entity, mut death_screen) in query.iter_mut() {
        death_screen.remove_timer.tick(time.delta());
        if death_screen.remove_timer.finished() {
            commands.entity(entity).despawn_recursive();
            next_state.set(AppState::MainMenu);
        }
    }
}

pub struct DeathScreenPlugin;

impl Plugin for DeathScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Death), spawn_death_screen)
            .add_systems(
                Update,
                remove_death_screen.run_if(in_state(AppState::Death)),
            );
    }
}
