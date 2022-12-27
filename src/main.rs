#![warn(clippy::manual_assert)]
#![warn(clippy::semicolon_if_nothing_returned)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

use bevy::prelude::*;

mod car;
mod spacetime;
mod grid;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0 / 5.0f32,
        })
        .insert_resource(Msaa::default())
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                present_mode: bevy::window::PresentMode::Immediate,
                ..default()
            },
            ..default()
        }))
        .add_plugin(car::CarPlugin)
        .add_plugin(spacetime::SpaceTimePlugin)
        .add_plugin(bevy_inspector_egui::WorldInspectorPlugin::new())
        .add_plugin(grid::GridPlugin)
        .add_system(bevy::window::close_on_esc)
        .run();
}
