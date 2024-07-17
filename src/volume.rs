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
use crate::persistent::Mixer;
use crate::state::AppState;
use crate::style::{ButtonId, h_space, hbox, text, text_button, v_space, wrapper};

#[derive(Component, Debug)]
struct VolumeControlUi;

pub fn volume_control_ui(
    asset_server: Res<AssetServer>,
    parent: &mut ChildBuilder,
    #[cfg(not(feature = "storage"))] mixer: Res<Mixer>,
    #[cfg(feature = "storage")] mixer: Res<bevy_persistent::Persistent<Mixer>>,
) {
    parent.spawn(hbox()).with_children(|parent| {
        parent.spawn(v_space(10.0));

        let (decrease_btn, decrease_btn_text) = text_button("-", 45);
        parent
            .spawn((wrapper(), ControlType::Keyboard))
            .with_children(|parent| {
                parent.spawn(decrease_btn).with_children(|parent| {
                    parent.spawn(decrease_btn_text);
                });
            });
        parent.spawn((
            ControlType::Gamepad,
            ImageBundle {
                style: Style {
                    width: Val::VMin(6.4),
                    height: Val::VMin(6.4),
                    ..default()
                },
                image: asset_server.load("ui/dpad_left.png").into(),
                ..default()
            },
        ));

        parent.spawn(h_space(1.0));
        parent.spawn((
            VolumeControlUi,
            text(format!("{:03.0}% Volume", mixer.master * 100.0)),
        ));
        parent.spawn(h_space(1.0));

        let (increase_btn, increase_btn_text) = text_button("+", 43);
        parent
            .spawn((wrapper(), ControlType::Keyboard))
            .with_children(|parent| {
                parent.spawn(increase_btn).with_children(|parent| {
                    parent.spawn(increase_btn_text);
                });
            });
        parent.spawn((
            ControlType::Gamepad,
            ImageBundle {
                style: Style {
                    width: Val::VMin(6.4),
                    height: Val::VMin(6.4),
                    ..default()
                },
                image: asset_server.load("ui/dpad_right.png").into(),
                ..default()
            },
        ));
    });
}

fn volume_control(
    #[cfg(not(feature = "storage"))] mut mixer: ResMut<Mixer>,
    #[cfg(feature = "storage")] mut mixer: ResMut<bevy_persistent::Persistent<Mixer>>,
    mut volume_ui: Query<&mut Text, With<VolumeControlUi>>,
    button_q: Query<(&Interaction, &ButtonId), (Changed<Interaction>, With<Button>)>,
    gamepads: Res<Gamepads>,
    buttons: Res<ButtonInput<GamepadButton>>,
) {
    let mut volume_ui = match volume_ui.get_single_mut() {
        Ok(volume_ui) => volume_ui,
        Err(_) => {
            return;
        }
    };

    for (interaction, button_id) in button_q.iter() {
        if *interaction == Interaction::Pressed {
            match button_id.id {
                45 => {
                    mixer.master = (mixer.master - 0.1).max(0.0);
                    volume_ui.sections[0].value = format!("{:03.0}% Volume", mixer.master * 100.0);
                    #[cfg(feature = "storage")]
                    mixer.persist().expect("failed to persist mixer");
                    return;
                }
                43 => {
                    mixer.master = (mixer.master + 0.1).min(1.0);
                    volume_ui.sections[0].value = format!("{:03.0}% Volume", mixer.master * 100.0);
                    #[cfg(feature = "storage")]
                    mixer.persist().expect("failed to persist mixer");
                    return;
                }
                _ => {}
            }
        }
    }

    for gamepad in gamepads.iter() {
        if buttons.just_pressed(GamepadButton {
            gamepad,
            button_type: GamepadButtonType::DPadLeft,
        }) {
            mixer.master = (mixer.master - 0.1).max(0.0);
            volume_ui.sections[0].value = format!("{:03.0}% Volume", mixer.master * 100.0);
            #[cfg(feature = "storage")]
            mixer.persist().expect("failed to persist mixer");
            return;
        }

        if buttons.just_pressed(GamepadButton {
            gamepad,
            button_type: GamepadButtonType::DPadRight,
        }) {
            mixer.master = (mixer.master + 0.1).min(1.0);
            volume_ui.sections[0].value = format!("{:03.0}% Volume", mixer.master * 100.0);
            #[cfg(feature = "storage")]
            mixer.persist().expect("failed to persist mixer");
            return;
        }
    }
}

fn volume_update(
    audio_q: Query<(&AudioSink, &PlaybackSettings)>,
    #[cfg(not(feature = "storage"))] mixer: Res<Mixer>,
    #[cfg(feature = "storage")] mixer: Res<bevy_persistent::Persistent<Mixer>>,
) {
    for (audio_sink, playback_settings) in audio_q.iter() {
        audio_sink.set_volume(playback_settings.volume.get() * mixer.master);
    }
}

pub struct VolumePlugin;

impl Plugin for VolumePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, volume_control.run_if(in_state(AppState::MainMenu)))
            .add_systems(Last, volume_update);
    }
}
