use bevy::prelude::*;
use bevy_infinite_grid::{InfiniteGrid, InfiniteGridBundle, InfiniteGridPlugin};

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InfiniteGridPlugin);
        app.add_startup_system(setup_system);
    }
}


fn setup_system(mut commands: Commands) {
    commands.spawn(InfiniteGridBundle {
        grid: InfiniteGrid {
            // shadow_color: None,
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Name::from("grid"));
}
