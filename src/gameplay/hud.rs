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

use crate::gameplay::health::Health;
use crate::gameplay::player::Player;
use crate::state::AppState;

#[derive(Component, Debug)]
struct Hud;

#[derive(Component, Debug)]
pub struct HealthBar;

fn spawn_ui(mut commands: Commands) {
    let container = NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Start,
            justify_content: JustifyContent::Start,
            padding: UiRect {
                left: Val::VMin(5.0),
                right: Val::VMin(5.0),
                top: Val::VMin(5.0),
                bottom: Val::VMin(5.0),
            },
            ..default()
        },
        ..default()
    };

    let health_bar_outer = NodeBundle {
        style: Style {
            width: Val::Vw(20.0),
            max_width: Val::Px(200.0),
            height: Val::Vh(5.0),
            max_height: Val::Px(20.0),
            padding: UiRect {
                left: Val::Px(2.0),
                right: Val::Px(2.0),
                top: Val::Px(2.0),
                bottom: Val::Px(2.0),
            },
            ..default()
        },
        background_color: Color::srgb(0.1, 0.1, 0.1).into(),
        ..default()
    };

    let health_bar_inner = NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        background_color: Color::srgb(0.8, 0.1, 0.1).into(),
        ..default()
    };

    commands.spawn((Hud, container)).with_children(|parent| {
        parent.spawn(health_bar_outer).with_children(|parent| {
            parent.spawn((HealthBar, health_bar_inner));
        });
    });
}

fn update_health_bar(
    mut query: Query<&mut Style, With<HealthBar>>,
    player_health_q: Query<&Health, With<Player>>,
) {
    let player_health = match player_health_q.iter().next() {
        Some(player_health) => player_health,
        None => return,
    };

    for mut style in query.iter_mut() {
        style.width = Val::Percent(player_health.health_percentage());
    }
}

fn despawn_hud(mut commands: Commands, query: Query<Entity, With<Hud>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), spawn_ui)
            .add_systems(Update, update_health_bar.run_if(in_state(AppState::InGame)))
            .add_systems(OnExit(AppState::InGame), despawn_hud);
    }
}
