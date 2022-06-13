pub const GRID_SIZE: (i16, i16) = (8, 8);
pub const GRID_CELL_SIZE: (i16, i16) = (32, 32);

pub const SCREEN_SIZE: (i32, i32) = (
    (GRID_SIZE.0 * GRID_CELL_SIZE.0) as i32,
    (GRID_SIZE.1 * GRID_CELL_SIZE.1) as i32,
);
