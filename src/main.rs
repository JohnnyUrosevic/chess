//! A small snake game done after watching
//! <https://www.youtube.com/watch?v=HCwMb0KslX8>
//! to showcase ggez and how it relates/differs from piston.
//!
//! Note that this example is meant to highlight the general
//! structure of a ggez game. Some of the details may need to
//! be changed to scale the game. For example, if we needed to
//! draw hundreds or thousands of shapes, a SpriteBatch is going
//! to offer far better performance than the direct draw calls
//! that this example uses.
//!
//! Author: @termhn
//! Original repo: https://github.com/termhn/ggez_snake

// First we'll import the crates we need for our game;
// in this case that is just `ggez` and `quad-rand`
extern crate good_web_game as ggez;

// Next we need to actually `use` the pieces of ggez that we are going
// to need frequently.
use ggez::{event, graphics, Context, GameResult};

const GRID_SIZE: (i16, i16) = (8, 8);
// Now we define the pixel size of each tile, which we make 32x32 pixels.
const GRID_CELL_SIZE: (i16, i16) = (32, 32);

// Next we define how large we want our actual window to be by multiplying
// the components of our grid size by its corresponding pixel size.
const SCREEN_SIZE: (i32, i32) = (
    (GRID_SIZE.0 * GRID_CELL_SIZE.0) as i32,
    (GRID_SIZE.1 * GRID_CELL_SIZE.1) as i32,
);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct GridPosition {
    x: i16,
    y: i16,
}

impl GridPosition {
    /// We make a standard helper function so that we can create a new `GridPosition`
    /// more easily.
    pub fn new(x: i16, y: i16) -> Self {
        GridPosition { x, y }
    }
}

/// We implement the `From` trait, which in this case allows us to convert easily between
/// a GridPosition and a ggez `graphics::Rect` which fills that grid cell.
/// Now we can just call `.into()` on a `GridPosition` where we want a
/// `Rect` that represents that grid cell.
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

/// And here we implement `From` again to allow us to easily convert between
/// `(i16, i16)` and a `GridPosition`.
impl From<(i16, i16)> for GridPosition {
    fn from(pos: (i16, i16)) -> Self {
        GridPosition { x: pos.0, y: pos.1 }
    }
}

struct GameState {
}

impl GameState {
    /// Our new function will set up the initial state of our game.
    pub fn new() -> Self {
        GameState {}
    }
}

/// Now we implement EventHandler for GameState. This provides an interface
/// that ggez will call automatically when different events happen.
impl event::EventHandler for GameState {
    /// Update will happen on every frame before it is drawn. This is where we update
    /// our game state to react to whatever is happening in the game world.
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }

    /// draw is where we should actually render the game's current state.
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());

        let dark_color = [0.0, 0.0, 1.0, 1.0].into();
        let light_color = [1.0, 1.0, 1.0, 1.0].into();


        for x in 0..GRID_SIZE.0 {
            for y in 0..GRID_SIZE.1 {
                let rect = GridPosition::new(x, y).into();

                let color = if (x + y) % 2 == 0 {dark_color} else {light_color};

                let rectangle =
                    graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, color)?;
                graphics::draw(ctx, &rectangle, (mint::Point2 { x: 0.0, y: 0.0 },))?;
            }
        }

        // Finally we call graphics::present to cycle the gpu's framebuffer and display
        // the new frame we just drew.
        graphics::present(ctx)?;
        // And return success.
        Ok(())
    }

}

fn main() -> GameResult {
    // First we create a new instance of our GameState struct, which implements EventHandler
    let state = GameState::new();
    // Then we create a configuration for how to start the application
    let conf = ggez::conf::Conf::default()
        // We set up the window. This title will be displayed in the title bar of the window.
        .window_title("Chess!".to_string())
        // Now we get to set the size of the window, which we use our SCREEN_SIZE constant from earlier to help with
        .window_width(SCREEN_SIZE.0)
        .window_height(SCREEN_SIZE.1);

    // And finally, we start the game!
    ggez::start(conf, |_context| Box::new(state))
}
