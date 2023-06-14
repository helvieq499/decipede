use bevy::prelude::*;

#[derive(Component)]
pub struct StartButton;

pub fn create(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Name::new("Menu"),
            NodeBundle::default(),
            bevy_ecss::StyleSheet::new(asset_server.load("style.css")),
            crate::state::MenuOnly,
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
    mut next_state: ResMut<NextState<crate::GameState>>,
) {
    for interaction in query.iter() {
        match interaction {
            Interaction::Clicked => next_state.set(crate::GameState::Playing),
            // todo: hover lighten background color
            _ => (),
        }
    }
}
