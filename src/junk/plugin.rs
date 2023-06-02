use bevy::prelude::*;

#[derive(Default)]
pub struct JunkPlugin();

impl Plugin for JunkPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<super::grid::JunkGrid>()
            .add_startup_system(super::tile::create_junk_tiles);
    }
}
