use bevy::{prelude::*, window::WindowResolution};

mod bullet;
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

// TODO: win condition
// TODO: slow down snakes (fps limit?)

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
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
        .add_state::<GameState>()
        .init_resource::<junk::grid::JunkGrid>()
        .add_systems(Startup, create_camera)
        .add_systems(OnEnter(GameState::Menu), menu::create)
        .add_systems(OnExit(GameState::Menu), state::cleanup::<state::MenuOnly>)
        .add_systems(
            OnEnter(GameState::Playing),
            (junk::tile::create, snake::create_initial, player::create),
        )
        .add_systems(
            OnExit(GameState::Playing),
            (junk::tile::cleanup, state::cleanup::<state::PlayingOnly>),
        )
        .add_systems(
            Update,
            (
                (menu::check_start_button).run_if(in_state(GameState::Menu)),
                (
                    player::update,
                    bullet::update_movement,
                    bullet::update_collision,
                    (snake::update_movement, snake::update_render).chain(),
                )
                    .run_if(in_state(GameState::Playing)),
            ),
        )
        .run();
}

fn create_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
