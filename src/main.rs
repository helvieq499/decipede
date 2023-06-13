use bevy::{prelude::*, window::WindowResolution};

mod junk;
mod menu;
mod snake;
mod utils;

pub const GRID_WIDTH: usize = 75;
pub const GRID_HEIGHT: usize = 50;
pub const CELL_WIDTH: usize = 16;
pub const CELL_HEIGHT: usize = 16;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Decipede".to_string(),
                        resolution: WindowResolution::new(
                            (GRID_WIDTH * CELL_WIDTH) as f32,
                            (GRID_HEIGHT * CELL_HEIGHT) as f32,
                        ),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(AssetPlugin {
                    watch_for_changes: true,
                    ..Default::default()
                }),
        )
        .add_plugin(bevy_ecss::EcssPlugin::with_hot_reload())
        .add_startup_system(create_camera)
        // state management & menu
        .add_state::<menu::GameState>()
        .add_system(menu::create.in_schedule(OnEnter(menu::GameState::Menu)))
        .add_system(menu::cleanup.in_schedule(OnExit(menu::GameState::Menu)))
        .add_system(menu::check_start_button.run_if(in_state(menu::GameState::Menu)))
        // junk tiles
        .init_resource::<junk::grid::JunkGrid>()
        .add_system(junk::tile::create_junk_tiles.in_schedule(OnEnter(menu::GameState::Playing)))
        .add_system(junk::tile::cleanup.in_schedule(OnExit(menu::GameState::Playing)))
        // snake
        .add_system(snake::create_initial_snake.in_schedule(OnEnter(menu::GameState::Playing)))
        .add_system(snake::snake_movement.run_if(in_state(menu::GameState::Playing)))
        .add_system(
            snake::snake_render
                .after(snake::snake_movement)
                .run_if(in_state(menu::GameState::Playing)),
        )
        .run();
}

fn create_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
