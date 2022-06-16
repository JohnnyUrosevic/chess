#![feature(option_result_contains)]
#![feature(let_chains)]

extern crate good_web_game as ggez;

pub mod util;
pub mod grid_position;
pub mod images;

use ggez::{event, graphics, Context, GameResult, input};
use shakmaty::{Chess, Position, Square};

use util::{GRID_SIZE, GRID_CELL_SIZE, SCREEN_SIZE};
use grid_position::{GridPosition};
use images::{Images};

use std::path;
use std::collections::{HashSet};


struct GameState {
    images: Images,
    position: Chess,
    selected: Option<Square>,
    preview_moves: HashSet<Square>
}

impl GameState {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let images = Images::new(ctx)?;
        let position = Chess::default();
        let selected = None;
        let preview_moves = HashSet::new();
        Ok(GameState {images, position, selected, preview_moves})
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());

        let dark_color = (60, 80, 245, 255).into();
        let light_color = (255, 255, 255, 255).into();
        let selected_color = (5, 67, 117, 255).into();

        let board = self.position.board();

        for x in 0..GRID_SIZE.0 {
            for y in 0..GRID_SIZE.1 {
                let grid_position = GridPosition::new(x, y);
                let rect = grid_position.into();
                let square = grid_position.into();

                let color = if self.selected.contains(&square) {
                    selected_color
                }
                else if (x + y) % 2 == 0 {
                    light_color
                }
                else {
                    dark_color
                };

                let rectangle =
                    graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, color)?;
                graphics::draw(ctx, &rectangle, (mint::Point2 { x: 0.0, y: 0.0 },))?;



                if let Some(piece) = board.piece_at(square) {
                    let image = self.images.piece_to_image.get(&piece).unwrap();

                    let pos: mint::Point2<f32> = grid_position.into();
                    let params = graphics::DrawParam::new()
                        .dest(pos)
                        .scale(mint::Vector2{x: 0.5, y: 0.5});
                    graphics::draw(ctx, image, params)?;
                }

                if self.preview_moves.contains(&square) {
                    let mut pos: mint::Point2<f32> = grid_position.into();

                    pos.x += (GRID_CELL_SIZE.0 / 2) as f32;
                    pos.y += (GRID_CELL_SIZE.1 / 2) as f32;

                    let circle =
                        graphics::Mesh::new_circle(ctx, graphics::DrawMode::fill(), pos, 20.0, 0.01, selected_color)?;
                    graphics::draw(ctx, &circle, (mint::Point2 { x: 0.0, y: 0.0 },))?;
                }
            }
        }

        graphics::present(ctx)?;
        Ok(())
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, _button: input::MouseButton, x: f32, y: f32) {
        let grid_x = x as u32 / GRID_CELL_SIZE.0;
        let grid_y = y as u32 / GRID_CELL_SIZE.1;

        let selected_square = GridPosition::new(grid_x, grid_y).into();

        // clicking the already selected square should remove the selection
        if let Some(selected) = self.selected && selected == selected_square {
            self.selected = None;
            self.preview_moves = HashSet::new();
            return;
        }

        let board = self.position.board();
        if let Some(piece) = board.piece_at(selected_square) && piece.color == self.position.turn() {
            self.selected = Some(selected_square);

            self.preview_moves = self.position.legal_moves()
                .iter()
                .filter(|x| x.from().contains(&selected_square))
                .map(|x| x.to())
                .collect();
        }
        else {
            if let Some(selected_move) = self.position.legal_moves()
                .iter()
                .find(|x| x.from() == self.selected && x.to() == selected_square) {
                    self.position.play_unchecked(selected_move);
            }
            self.selected = None;
            self.preview_moves = HashSet::new();
        }

    }
}

fn main() -> GameResult {
    let resource_dir = path::PathBuf::from("./resources");

    let conf = ggez::conf::Conf::default()
        // We set up the window. This title will be displayed in the title bar of the window.
        .window_title("Chess!".to_string())
        // Now we get to set the size of the window, which we use our SCREEN_SIZE constant from earlier to help with
        .window_width(SCREEN_SIZE.0)
        .window_height(SCREEN_SIZE.1)
        .physical_root_dir(Some(resource_dir));

    // And finally, we start the game!
    ggez::start(conf, |mut context|{
        Box::new(GameState::new(&mut context).unwrap())
    })
}
