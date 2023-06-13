use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Snake {
    pub x: usize,
    pub y: usize,
    pub grow: i8,
    pub is_going_left: bool,
}

impl Snake {
    pub fn new(x: usize, y: usize, grow: i8, is_going_left: bool) -> Self {
        Self {
            x,
            y,
            grow,
            is_going_left,
        }
    }
}

pub fn create_initial(mut commands: Commands) {
    create(
        &mut commands,
        (crate::GRID_WIDTH / 2) as usize,
        0,
        20,
        rand::random(),
    );
}

pub fn create(commands: &mut Commands, x: usize, y: usize, grow: i8, is_going_left: bool) {
    let (fx, fy) = crate::utils::grid_to_pixels(x, y);

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::LIME_GREEN,
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3 {
                    x: fx,
                    y: fy,
                    z: 0.,
                },
                scale: Vec3 {
                    x: crate::CELL_WIDTH as f32,
                    y: crate::CELL_HEIGHT as f32,
                    z: 1.,
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Snake::new(x, y, grow, is_going_left));
}

pub fn update_movement(
    mut commands: Commands,
    junk: Res<crate::junk::grid::JunkGrid>,
    mut snakes: Query<(Entity, &mut Snake)>,
    mut next_state: ResMut<NextState<crate::menu::GameState>>,
) {
    let snake_count = snakes.iter().count();

    for (entity, mut snake) in snakes.iter_mut() {
        let hitting_wall = if snake.is_going_left {
            snake.x < 1 || junk.0[snake.x - 1][snake.y]
        } else {
            snake.x + 1 >= crate::GRID_WIDTH || junk.0[snake.x + 1][snake.y]
        };

        if snake.grow != 0 {
            create(
                &mut commands,
                snake.x,
                snake.y,
                snake.grow - 1,
                snake.is_going_left,
            );
            snake.grow = 0;
        }

        if hitting_wall {
            snake.y += 1;
            snake.is_going_left ^= true;
        } else {
            if snake.is_going_left {
                snake.x -= 1;
            } else {
                snake.x += 1;
            }
        }

        if snake.y >= crate::GRID_HEIGHT {
            // you lost
            commands.entity(entity).despawn();

            if snake_count == 1 {
                next_state.set(crate::menu::GameState::Menu);
            }
        }
    }
}

pub fn update_render(mut snakes: Query<(&Snake, &mut Transform)>) {
    for (snake, mut transform) in snakes.iter_mut() {
        (transform.translation.x, transform.translation.y) =
            crate::utils::grid_to_pixels(snake.x, snake.y);
    }
}
