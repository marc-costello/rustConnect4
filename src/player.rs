#[derive(Copy, Clone, Debug)]
pub struct Player {
    pub player_type: PlayerType,
    pub player_char: char
}

impl PartialEq for Player {
    fn eq(&self, other: &Player) -> bool { 
        self.player_char == other.player_char
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PlayerType {
    Human,
    Computer,
}