const X_OFFSET: f32 = (crate::GRID_WIDTH as f32 - 1.) / 2.;
const Y_OFFSET: f32 = (crate::GRID_HEIGHT as f32 - 1.) / 2.;

pub fn grid_to_pixels(x: usize, y: usize) -> (f32, f32) {
    (
        ((x as f32 - X_OFFSET) * crate::CELL_WIDTH as f32),
        ((y as f32 - Y_OFFSET) * -(crate::CELL_HEIGHT as isize) as f32),
    )
}

pub fn pixels_to_grid(x: f32, y: f32) -> (usize, usize) {
    (
        (x / crate::CELL_WIDTH as f32 + X_OFFSET) as usize,
        (y / -(crate::CELL_WIDTH as f32) + Y_OFFSET) as usize,
    )
}

#[test]
fn test_grid_conversion() {
    let convert = |x, y| {
        let (fx, fy) = grid_to_pixels(x, y);
        pixels_to_grid(fx, fy)
    };

    assert_eq!(convert(11, 23), (11, 23));
}

pub fn invert_if<T>(num: T, condition: bool) -> T
where
    T: std::ops::Neg<Output = T>,
{
    if condition {
        -num
    } else {
        num
    }
}
