use crate::player::Player;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Coin {
    Player(Player),
    Empty
}