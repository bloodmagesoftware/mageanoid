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

extern crate bevy;

use bevy::prelude::*;

use crate::state::AppState;
use crate::style::*;

pub struct MainMenuPlugin;

#[derive(Component, Debug)]
struct MainMenu;

fn spawn_ui(mut commands: Commands) {
    let container = NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        ..default()
    };

    let (start_btn, start_btn_text) = text_button("Start", 0);
    let (exit_btn, exit_btn_text) = text_button("Exit", 1);

    commands
        .spawn((MainMenu, container))
        .with_children(|parent| {
            parent.spawn(text_title("Mageanoid"));
            parent.spawn(v_space(20.0));
            parent.spawn(start_btn).with_children(|parent| {
                parent.spawn(start_btn_text);
            });
            parent.spawn(exit_btn).with_children(|parent| {
                parent.spawn(exit_btn_text);
            });
        });
}

fn on_button_click(
    mut next_state: ResMut<NextState<AppState>>,
    mut app_exit_events: ResMut<Events<AppExit>>,
    button_q: Query<(&Interaction, &ButtonId), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, button_id) in button_q.iter() {
        if *interaction == Interaction::Pressed {
            match button_id.id {
                0 => next_state.set(AppState::InGame),
                1 => {
                    app_exit_events.send(AppExit::Success);
                }
                _ => (),
            }
        }
    }
}

fn despawn_ui(mut commands: Commands, query: Query<Entity, With<MainMenu>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), spawn_ui)
            .add_systems(Update, on_button_click.run_if(in_state(AppState::MainMenu)))
            .add_systems(OnExit(AppState::MainMenu), despawn_ui);
    }
}
