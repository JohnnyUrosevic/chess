use ggez::{graphics};
use shakmaty::{Square, File, Rank};

use crate::util::{GRID_CELL_SIZE};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct GridPosition {
    x: u32,
    y: u32,
}

impl GridPosition {
    pub fn new(x: u32, y: u32) -> Self {
        GridPosition { x, y }
    }
}


impl From<GridPosition> for graphics::Rect {
    fn from(pos: GridPosition) -> Self {
        graphics::Rect::new_i32(
            pos.x as i32 * GRID_CELL_SIZE.0 as i32,
            pos.y as i32 * GRID_CELL_SIZE.1 as i32,
            GRID_CELL_SIZE.0 as i32,
            GRID_CELL_SIZE.1 as i32,
        )
    }
}

impl From<GridPosition> for mint::Point2<f32> {
    fn from(pos: GridPosition) -> Self {
        mint::Point2 {
            x: pos.x as f32 * GRID_CELL_SIZE.0 as f32,
            y: pos.y as f32 * GRID_CELL_SIZE.1 as f32,
        }
    }
}

impl From<GridPosition> for Square {
    fn from(pos: GridPosition) -> Self {
        let file_char = ('A'..='H')
            .nth(pos.x as usize)
            .expect("X out of bounds");
        let file = File::from_char(file_char).unwrap();

        let rank = Rank::new(pos.y as u32);

        Square::from_coords(file, rank)
    }
}

impl From<(u32, u32)> for GridPosition {
    fn from(pos: ( u32, u32)) -> Self {
        GridPosition { x: pos.0, y: pos.1 }
    }
}

impl From<Square> for GridPosition {
    fn from(square: Square) -> Self {
        let (file, rank) = square.coords();

        let file_char = file.char();
        let x = ('A'..='H')
            .position(|c| c == file_char)
            .unwrap();

        GridPosition { x: x as u32, y: rank as u32 }
    }
}
