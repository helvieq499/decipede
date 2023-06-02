use bevy::prelude::*;
use rand::Rng;

const JUNK_AMOUNT: usize = 25;
const JUNK_SCREEN: f32 = 0.8;

#[derive(Component, Default)]
pub struct JunkTile {}

pub fn create_junk_tiles(mut commands: Commands, mut grid: ResMut<super::grid::JunkGrid>) {
    debug_assert!(JUNK_SCREEN > 0., "screen junk ratio too low");
    debug_assert!(JUNK_SCREEN <= 1., "screen junk ratio too high");

    const SEGMENT_SIZE: usize = ((crate::GRID_WIDTH * crate::GRID_HEIGHT) as f32
        / (JUNK_AMOUNT as f32 / JUNK_SCREEN)) as usize;

    let mut rng = rand::thread_rng();

    for i in 0..JUNK_AMOUNT {
        let position = (rng.gen::<usize>() % SEGMENT_SIZE) + (i * SEGMENT_SIZE);

        let x = position % crate::GRID_WIDTH as usize;

        create_junk_tile(
            &mut commands,
            &mut grid,
            x,
            (position - x) / crate::GRID_WIDTH as usize,
        );
    }
}

pub fn create_junk_tile(
    commands: &mut Commands,
    grid: &mut super::grid::JunkGrid,
    x: usize,
    y: usize,
) {
    if std::mem::replace(&mut grid.0[x as usize][y as usize], true) {
        return;
    }

    let (fx, fy) = crate::utils::grid_to_pixels(x, y);

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::MIDNIGHT_BLUE,
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3 {
                x: fx,
                y: fy,
                z: 0.,
            },
            scale: Vec3::new(crate::CELL_WIDTH as f32, crate::CELL_HEIGHT as f32, 1.),
            ..Default::default()
        },
        ..Default::default()
    });
}
