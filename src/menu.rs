use bevy::prelude::*;

#[derive(Clone, Debug, Default, Hash, States, PartialEq, Eq)]
pub enum GameState {
    #[default]
    Menu,
    Playing,
}

#[derive(Component)]
pub struct MenuMarker;

#[derive(Component)]
pub struct StartButton;

pub fn create(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Name::new("Menu"),
            NodeBundle::default(),
            bevy_ecss::StyleSheet::new(asset_server.load("style.css")),
            MenuMarker,
        ))
        .with_children(|parent| {
            parent
                .spawn((ButtonBundle::default(), StartButton))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section("Start", TextStyle::default()));
                });
        });
}

pub fn check_start_button(
    query: Query<&Interaction, (With<StartButton>, Changed<Interaction>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for interaction in query.iter() {
        match interaction {
            Interaction::Clicked => next_state.set(GameState::Playing),
            // todo: hover lighten background color
            _ => (),
        }
    }
}

pub fn cleanup(mut commands: Commands, menu_root: Query<Entity, With<MenuMarker>>) {
    menu_root.iter().for_each(|entity| {
        commands.entity(entity).despawn_recursive();
    });
}
