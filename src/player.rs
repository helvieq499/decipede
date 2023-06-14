use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

pub fn create(mut commands: Commands) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(
                0.,
                -crate::Y_EXTENT + 5. * crate::CELL_HEIGHT as f32,
                1.,
            ))
            .with_scale(Vec3::new(
                crate::CELL_WIDTH as f32,
                crate::CELL_HEIGHT as f32,
                1.,
            )),
            ..Default::default()
        })
        .insert((Player, crate::state::PlayingOnly));
}

pub fn update(mut query: Query<&mut Transform, With<Player>>, keyboard: Res<Input<KeyCode>>) {
    query.for_each_mut(|mut transform| {
        let left = keyboard.pressed(KeyCode::Left) || keyboard.pressed(KeyCode::A);
        let right = keyboard.pressed(KeyCode::Right) || keyboard.pressed(KeyCode::D);
        if left != right {
            transform.translation.x = (transform.translation.x + crate::utils::invert_if(4., left))
                .clamp(-crate::X_EXTENT, crate::X_EXTENT);
        }

        let up = keyboard.pressed(KeyCode::Up) || keyboard.pressed(KeyCode::W);
        let down = keyboard.pressed(KeyCode::Down) || keyboard.pressed(KeyCode::S);
        if up != down {
            transform.translation.y = (transform.translation.y + crate::utils::invert_if(2., down))
                .clamp(
                    -crate::Y_EXTENT,
                    -crate::Y_EXTENT + 5. * crate::CELL_HEIGHT as f32,
                );
        }
    });
}
