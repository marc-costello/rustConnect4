use std::rc::Rc;
use crate::player::Player;

#[derive(Debug, PartialEq)]
pub struct Move {
    pub x: usize,
    pub player: Rc<Player>,
}

impl Move {
    pub fn new(player: Rc<Player>, col: u8) -> Move {
        Move {
            x: usize::from(col),
            player: player
        }
    }

    pub fn player(&self) -> &Player {
        self.player.as_ref()
    }
}