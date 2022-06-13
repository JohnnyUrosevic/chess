extern crate good_web_game as ggez;

pub mod util;
pub mod grid_position;
pub mod images;

use ggez::{event, graphics, Context, GameResult};
use shakmaty::{Chess, Position};

use util::{GRID_SIZE, SCREEN_SIZE};
use grid_position::{GridPosition};
use images::{Images};

use std::path;

struct GameState {
    images: Images,
    position: Chess,
}

impl GameState {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let images = Images::new(ctx)?;
        let position = Chess::default();
        Ok(GameState {images, position})
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());

        let dark_color = [0.0, 0.0, 1.0, 1.0].into();
        let light_color = [1.0, 1.0, 1.0, 1.0].into();

        let board = self.position.board();

        for x in 0..GRID_SIZE.0 {
            for y in 0..GRID_SIZE.1 {
                let grid_position = GridPosition::new(x, y);
                let rect = grid_position.into();

                let color = if (x + y) % 2 == 0 {light_color} else {dark_color};

                let rectangle =
                    graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, color)?;
                graphics::draw(ctx, &rectangle, (mint::Point2 { x: 0.0, y: 0.0 },))?;

                let square = grid_position.into();
                if let Some(piece) = board.piece_at(square) {
                    let image = self.images.piece_to_image.get(&piece).unwrap();

                    let pos: mint::Point2<f32> = grid_position.into();
                    let params = graphics::DrawParam::new()
                        .dest(pos)
                        .scale(mint::Vector2{x: 0.5, y: 0.5});
                    graphics::draw(ctx, image, params)?;
                }
            }
        }

        graphics::present(ctx)?;
        Ok(())
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
    ggez::start(conf, |mut context| Box::new(GameState::new(&mut context).unwrap()))
}
