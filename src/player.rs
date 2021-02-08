use std::fmt;
use std::fmt::Display;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Player {
    Red,
    Yellow,
}

impl Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let char = match self {
            Player::Red => 'R',
            Player::Yellow => 'Y',
        };
        write!(f, "{}", char)
    }
}