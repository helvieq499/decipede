use bevy::{prelude::*, window::WindowResolution};

mod junk;
mod snake;
mod utils;

pub const GRID_WIDTH: usize = 75;
pub const GRID_HEIGHT: usize = 50;
pub const CELL_WIDTH: usize = 16;
pub const CELL_HEIGHT: usize = 16;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Decipede".to_string(),
            resolution: WindowResolution::new(
                (GRID_WIDTH * CELL_WIDTH) as f32,
                (GRID_HEIGHT * CELL_HEIGHT) as f32,
            ),
            ..Default::default()
        }),
        ..Default::default()
    }))
    .add_plugin(junk::plugin::JunkPlugin::default())
    .add_startup_system(create_camera)
    .add_startup_system(snake::create_initial_snake)
    .add_system(snake::snake_movement)
    .add_system(snake::snake_render.after(snake::snake_movement));

    #[cfg(feature = "dev")]
    app.add_plugin(bevy_editor_pls::EditorPlugin::default());

    app.run();
}

fn create_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
