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

use crate::controls::ControlType;
use crate::persistent::Score;
use crate::state::AppState;
use crate::style::{ButtonId, text, text_button, text_title, v_space};

#[derive(Component, Debug)]
struct PauseMenu;

fn spawn_pause_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    #[cfg(feature = "storage")] score: Res<bevy_persistent::Persistent<Score>>,
    #[cfg(not(feature = "storage"))] score: Res<Score>,
) {
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

    let (resume_btn, resume_btn_text) = text_button("Resume", 0);
    let (main_menu_btn, main_menu_btn_text) = text_button("Main Menu", 1);

    commands
        .spawn((PauseMenu, container))
        .with_children(|parent| {
            parent.spawn(text_title("Mageanoid"));
            parent.spawn(text(format!("Score: {}", score.current_score)));
            parent.spawn(v_space(20.0));

            parent
                .spawn(NodeBundle {
                    style: Style {
                        display: Display::Flex,
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        flex_direction: FlexDirection::Row,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(resume_btn).with_children(|parent| {
                        parent.spawn(resume_btn_text);
                    });
                    parent.spawn((
                        ControlType::Gamepad,
                        ImageBundle {
                            image: asset_server.load("ui/face_north.png").into(),
                            style: Style {
                                width: Val::VMin(6.4),
                                height: Val::VMin(6.4),
                                ..default()
                            },
                            ..default()
                        },
                    ));
                });

            parent.spawn(v_space(5.0));

            parent
                .spawn(NodeBundle {
                    style: Style {
                        display: Display::Flex,
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        flex_direction: FlexDirection::Row,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(main_menu_btn).with_children(|parent| {
                        parent.spawn(main_menu_btn_text);
                    });

                    parent.spawn((
                        ControlType::Gamepad,
                        ImageBundle {
                            image: asset_server.load("ui/face_south.png").into(),
                            style: Style {
                                width: Val::VMin(6.4),
                                height: Val::VMin(6.4),
                                ..default()
                            },
                            ..default()
                        },
                    ));
                });
        });
}

fn on_button_click(
    mut next_state: ResMut<NextState<AppState>>,
    button_q: Query<(&Interaction, &ButtonId), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, button_id) in button_q.iter() {
        if *interaction == Interaction::Pressed {
            match button_id.id {
                0 => next_state.set(AppState::InGame),
                1 => next_state.set(AppState::MainMenu),
                _ => {}
            }
        }
    }
}

fn despawn_pause_menu(mut commands: Commands, query: Query<Entity, With<PauseMenu>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn toggle_pause(
    state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
    keys: Res<ButtonInput<KeyCode>>,
    gamepad: Res<Gamepads>,
    buttons: Res<ButtonInput<GamepadButton>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        next_state.set(match state.get() {
            AppState::Paused => AppState::InGame,
            _ => AppState::Paused,
        });
        return;
    }

    for gamepad in gamepad.iter() {
        let start_button = GamepadButton {
            gamepad,
            button_type: GamepadButtonType::Start,
        };
        if buttons.just_pressed(start_button) {
            next_state.set(match state.get() {
                AppState::Paused => AppState::InGame,
                _ => AppState::Paused,
            });
            return;
        }

        if state.get() == &AppState::Paused {
            if buttons.just_pressed(GamepadButton {
                gamepad,
                button_type: GamepadButtonType::South,
            }) {
                next_state.set(AppState::MainMenu);
                return;
            }

            if buttons.just_pressed(GamepadButton {
                gamepad,
                button_type: GamepadButtonType::North,
            }) {
                next_state.set(AppState::InGame);
                return;
            }
        };
    }
}

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Paused), spawn_pause_menu)
            .add_systems(Update, on_button_click.run_if(in_state(AppState::Paused)))
            .add_systems(
                Update,
                toggle_pause.run_if(in_state(AppState::InGame).or_else(in_state(AppState::Paused))),
            )
            .add_systems(OnExit(AppState::Paused), despawn_pause_menu);
    }
}
