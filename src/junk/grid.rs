use bevy::prelude::*;

#[derive(Resource)]
pub struct JunkGrid(pub [[bool; crate::GRID_HEIGHT as usize]; crate::GRID_WIDTH as usize]);

impl Default for JunkGrid {
    fn default() -> Self {
        Self([[false; crate::GRID_HEIGHT as usize]; crate::GRID_WIDTH as usize])
    }
}
