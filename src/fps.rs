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

/// Marker to find the container entity so we can show/hide the FPS counter
#[derive(Component)]
struct FpsRoot;

/// Marker to find the text entity so we can update it
#[derive(Component)]
struct FpsText {
    buffer: CircularBuffer,
}

impl Default for FpsText {
    fn default() -> Self {
        FpsText {
            buffer: CircularBuffer::new(),
        }
    }
}

fn setup_fps_counter(mut commands: Commands) {
    // create our UI root node
    // this is the wrapper/container for the text
    let root = commands
        .spawn((
            FpsRoot,
            NodeBundle {
                // give it a dark background for readability
                background_color: BackgroundColor(Color::BLACK.with_a(0.5)),
                // make it "always on top" by setting the Z index to maximum
                // we want it to be displayed over all other UI
                z_index: ZIndex::Global(i32::MAX),
                style: Style {
                    position_type: PositionType::Absolute,
                    // position it at the top-right corner
                    // 1% away from the top window edge
                    right: Val::Percent(1.),
                    top: Val::Percent(1.),
                    // set bottom/left to Auto, so it can be
                    // automatically sized depending on the text
                    bottom: Val::Auto,
                    left: Val::Auto,
                    // give it some padding for readability
                    padding: UiRect::all(Val::Px(4.0)),
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .id();
    // create our text
    let text_fps = commands
        .spawn((
            FpsText::default(),
            TextBundle {
                // use two sections, so it is easy to update just the number
                text: Text::from_sections([
                    TextSection {
                        value: "FPS: ".into(),
                        style: TextStyle {
                            font_size: 16.0,
                            color: Color::WHITE,
                            // if you want to use your game's font asset,
                            // uncomment this and provide the handle:
                            // font: my_font_handle
                            ..default()
                        },
                    },
                    TextSection {
                        value: " N/A".into(),
                        style: TextStyle {
                            font_size: 16.0,
                            color: Color::WHITE,
                            // if you want to use your game's font asset,
                            // uncomment this and provide the handle:
                            // font: my_font_handle
                            ..default()
                        },
                    },
                ]),
                ..Default::default()
            },
        ))
        .id();
    commands.entity(root).push_children(&[text_fps]);
}

fn fps_text_update_system(time: Res<Time>, mut query: Query<(&mut Text, &mut FpsText)>) {
    for (mut ui_text, mut fps_comp) in query.iter_mut() {
        // push current frame time to the buffer
        fps_comp.buffer.add(time.delta_seconds());

        // update the FPS counter text
        ui_text.sections[1].value = format!("{:.0}", 1.0 / fps_comp.buffer.median().unwrap_or(1.0));
    }
}

/// Toggle the FPS counter when pressing F12
fn fps_counter_showhide(
    mut q: Query<&mut Visibility, With<FpsRoot>>,
    kbd: Res<ButtonInput<KeyCode>>,
) {
    if kbd.just_pressed(KeyCode::F12) {
        let mut vis = q.single_mut();
        *vis = match *vis {
            Visibility::Hidden => Visibility::Visible,
            _ => Visibility::Hidden,
        };
    }
}

pub struct FpsPlugin;

impl Plugin for FpsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_fps_counter);
        app.add_systems(Update, (fps_text_update_system, fps_counter_showhide));
    }
}

struct CircularBuffer {
    buffer: [f32; 32],
    head: usize,
    size: usize,
}

impl CircularBuffer {
    fn new() -> Self {
        CircularBuffer {
            buffer: [0.0; 32],
            head: 0,
            size: 0,
        }
    }

    fn add(&mut self, value: f32) {
        self.buffer[self.head] = value;
        self.head = (self.head + 1) % self.buffer.len();
        if self.size < self.buffer.len() {
            self.size += 1;
        }
    }

    fn median(&self) -> Option<f32> {
        if self.size == 0 {
            return None;
        }

        // Collect the elements in the buffer
        let mut elements: Vec<f32> = self.buffer.iter().take(self.size).cloned().collect();

        // Sort the elements
        elements.sort_by(|a, b| a.partial_cmp(b).unwrap());

        // Calculate the median
        let mid = self.size / 2;
        if self.size % 2 == 0 {
            Some((elements[mid - 1] + elements[mid]) / 2.0)
        } else {
            Some(elements[mid])
        }
    }
}
