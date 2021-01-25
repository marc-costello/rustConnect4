pub mod player;
pub mod renderer;
pub mod checks;
pub mod game;
pub mod r#move;
pub mod board;

use crate::game::Game;

fn main() {
    let mut game = Game::new_game();
    game.start()
}