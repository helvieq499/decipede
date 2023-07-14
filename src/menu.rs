use bevy::prelude::*;

#[derive(Component)]
pub struct StartButton;

pub fn create(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Name::new("Menu"),
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                ..Default::default()
            },
            crate::state::MenuOnly,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(150.),
                            height: Val::Px(75.),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        background_color: Color::rgb_u8(0x11, 0x11, 0x11).into(),
                        ..Default::default()
                    },
                    StartButton,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Start",
                        TextStyle {
                            color: Color::WHITE,
                            font_size: 40.,
                            font: asset_server.load("OpenSans-Regular.ttf"),
                        },
                    ));
                });
        });
}

pub fn check_start_button(
    query: Query<&Interaction, (With<StartButton>, Changed<Interaction>)>,
    mut next_state: ResMut<NextState<crate::GameState>>,
) {
    for interaction in query.iter() {
        match interaction {
            Interaction::Pressed => next_state.set(crate::GameState::Playing),
            // todo: hover lighten background color
            _ => (),
        }
    }
}
