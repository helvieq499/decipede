pub use bevy::prelude::*;

#[derive(Clone, Debug, Default, Hash, States, PartialEq, Eq)]
pub enum GameState {
    #[default]
    Menu,
    Playing,
}

#[derive(Component)]
pub struct MenuOnly;

#[derive(Component)]
pub struct PlayingOnly;

pub fn cleanup<T>(mut commands: Commands, query: Query<Entity, With<T>>)
where
    T: Component,
{
    query.for_each(|entity| commands.entity(entity).despawn_recursive());
}
