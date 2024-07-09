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

pub const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.35, 0.35);

fn update_buttons(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

#[derive(Component, Debug)]
pub struct ButtonId {
    pub id: u8,
}

#[derive(Bundle, Debug)]
pub struct InteractiveButtonBundle {
    pub button: ButtonBundle,
    pub id: ButtonId,
}

pub fn text(content: impl Into<String>) -> TextBundle {
    TextBundle::from_section(
        content,
        TextStyle {
            font_size: 32.0,
            color: Color::WHITE,
            ..default()
        },
    )
}

pub fn text_title(content: impl Into<String>) -> TextBundle {
    TextBundle::from_section(
        content,
        TextStyle {
            font_size: 75.0,
            color: Color::WHITE,
            ..default()
        },
    )
}

pub fn v_space(height: f32) -> NodeBundle {
    NodeBundle {
        style: Style {
            height: Val::Vh(height),
            ..default()
        },
        ..default()
    }
}

pub fn text_button(content: impl Into<String>, id: u8) -> (InteractiveButtonBundle, TextBundle) {
    let button = InteractiveButtonBundle {
        button: ButtonBundle {
            background_color: NORMAL_BUTTON.into(),
            style: Style {
                width: Val::Px(256.0),
                height: Val::Px(64.0),
                border: UiRect::all(Val::Px(5.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        },
        id: ButtonId { id },
    };

    let button_text = TextBundle::from_section(
        content,
        TextStyle {
            font_size: 40.0,
            color: Color::WHITE,
            ..default()
        },
    );

    (button, button_text)
}

pub struct StylePlugin;

impl Plugin for StylePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_buttons);
    }
}
