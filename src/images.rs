use std::{collections::{HashMap}};

use ggez::{graphics};
use ggez::{Context, GameResult};
use shakmaty::{Color, Role};

pub struct Images {
    piece_to_image: HashMap<(Color, Role), graphics::Image>,
}

impl Images {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let mut map = HashMap::new();
        for role_char in ['B', 'K', 'N', 'P', 'Q', 'R'].iter() {
            let role = match role_char {
                'B' => Role::Bishop,
                'K' => Role::King,
                'N' => Role::Knight,
                'P' => Role::Pawn,
                'Q' => Role::Queen,
                'R' => Role::Rook,
                _ => unreachable!(),
            };

            let white_image = graphics::Image::new(ctx, format!("w{}.png", role_char))?;
            map.insert((Color::White, role), white_image);

            let black_image = graphics::Image::new(ctx, format!("b{}.png", role_char))?;
            map.insert((Color::Black, role), black_image);
        }

        Ok(Images {piece_to_image: map})
    }
}