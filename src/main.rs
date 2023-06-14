use bevy::{prelude::*, window::WindowResolution};

mod junk;
mod menu;
mod player;
mod snake;
mod utils;

mod state;
pub use state::GameState;

pub const GRID_WIDTH: usize = 75;
pub const GRID_HEIGHT: usize = 50;
pub const CELL_WIDTH: usize = 16;
pub const CELL_HEIGHT: usize = 16;
pub const X_EXTENT: f32 = ((crate::GRID_WIDTH - 1) * crate::CELL_WIDTH / 2) as f32;
pub const Y_EXTENT: f32 = ((crate::GRID_HEIGHT - 1) * crate::CELL_HEIGHT / 2) as f32;

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
        // state management
        .add_state::<GameState>()
        .add_system(state::cleanup::<state::MenuOnly>.in_schedule(OnExit(GameState::Menu)))
        .add_system(state::cleanup::<state::PlayingOnly>.in_schedule(OnExit(GameState::Playing)))
        // menu
        .add_system(menu::create.in_schedule(OnEnter(GameState::Menu)))
        .add_system(menu::check_start_button.run_if(in_state(GameState::Menu)))
        // junk tiles
        .init_resource::<junk::grid::JunkGrid>()
        .add_system(junk::tile::create.in_schedule(OnEnter(GameState::Playing)))
        .add_system(junk::tile::cleanup.in_schedule(OnExit(GameState::Playing)))
        // snake
        .add_system(snake::create_initial.in_schedule(OnEnter(GameState::Playing)))
        .add_system(snake::update_movement.run_if(in_state(GameState::Playing)))
        .add_system(
            snake::update_render
                .after(snake::update_movement)
                .run_if(in_state(GameState::Playing)),
        )
        // player
        .add_system(player::create.in_schedule(OnEnter(GameState::Playing)))
        .add_system(player::update.run_if(in_state(GameState::Playing)))
        .run();
}

fn create_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
