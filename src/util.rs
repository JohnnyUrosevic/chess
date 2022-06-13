pub const GRID_SIZE: (u32, u32) = (8, 8);
pub const GRID_CELL_SIZE: (u32, u32) = (90, 90);

pub const SCREEN_SIZE: (i32, i32) = (
    (GRID_SIZE.0 * GRID_CELL_SIZE.0) as i32,
    (GRID_SIZE.1 * GRID_CELL_SIZE.1) as i32,
);
