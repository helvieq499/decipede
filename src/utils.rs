pub fn grid_to_pixels(x: usize, y: usize) -> (f32, f32) {
    const X_OFFSET: f32 = (crate::GRID_WIDTH as f32 - 1.) / 2.;
    const Y_OFFSET: f32 = (crate::GRID_HEIGHT as f32 - 1.) / 2.;

    (
        ((x as f32 - X_OFFSET) * crate::CELL_WIDTH as f32),
        ((y as f32 - Y_OFFSET) * -(crate::CELL_HEIGHT as isize) as f32),
    )
}
