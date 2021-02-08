use rand::{thread_rng, Rng};
use std::error::Error;
use std::io::{Error as ioError, ErrorKind};
use crate::player::Player;
use crate::board::{self, Board};
use crate::coin::Coin;
use crate::checks::{is_vertical_win, is_horizontal_win, is_diagonal_win, is_draw};
use crate::renderer;

pub enum GameOutcome {
    Win(Player),
    Draw
}

pub struct Game { 
    player_1: Player,
    player_2: Player,
    board: Board,
    current_player: Player
 }

 impl Game {
     pub fn new_game() -> Game {
         Game {
             player_1: Player::Red,
             player_2: Player::Yellow,
             current_player: Player::Red,
             board: board::new_board(),
         }
     }

     pub fn next_player(&mut self) -> &Self {
         self.current_player = if self.current_player == self.player_1 {
             self.player_2
         } else {
             self.player_1
         };
         self
     }

     pub fn start(&mut self) {
        renderer::render_board(&self.board);
         
         // game loop 
         let outcome: Option<Player> = loop {
             if self.current_player == self.player_1 {
                match human_move(&self.board) {
                    Ok(col) => board::drop_coin(&mut self.board, Coin::Player(self.player_1), col.into()),
                    Err(_) => continue,
                };
             } else {
                let computer_move = computer_move(&self.board);
                board::drop_coin(&mut self.board, Coin::Player(self.player_2), computer_move.into());

                // render out the board with the new move
                renderer::render_board(&self.board);
             }
             
             // run win check.
             match self.check_for_winner() {
                 Some(GameOutcome::Win(player)) => break Some(player),
                 Some(GameOutcome::Draw) => break None,
                 None => self.next_player(),
             };
         };

         if let Some(player) = outcome {
            // announce winner
            println!("{:?} HAS WON!", player);
         } else {
            // draw
            println!("THIS GAME IS A DRAW!");
         }
     }

     fn check_for_winner(&self) -> Option<GameOutcome> {
        let checks_for_player = |player| {
            let vertical = is_vertical_win(&self.board, player);
            let horizontal = is_horizontal_win(&self.board, player);
            let diagonal = is_diagonal_win(&self.board, player);
            match (vertical, horizontal, diagonal) {
                (Some(_),_,_) | 
                (_,Some(_),_) | 
                (_,_,Some(_)) => Some(GameOutcome::Win(player)),
                _ => None
            }
        };
        
        let is_p1_win = checks_for_player(self.player_1);
        if is_p1_win.is_some() {
            return is_p1_win
        }
        let is_p2_win = checks_for_player(self.player_2);
        if is_p2_win.is_some() {
            return is_p2_win
        }
        if is_draw(&self.board) { Some(GameOutcome::Draw) } else { None }
     }
 }

fn human_move(_board: &Board) -> Result<u8, Box<dyn Error>> {
    let input = renderer::read()?;
    let input_int = match input.trim().parse::<u8>() {
        Ok(i) if i < 1 => 
            return Err(Box::new(ioError::new(ErrorKind::InvalidInput, "Input cannot be less than 1"))),
        Ok(i) => i,
        Err(e) => return Err(Box::new(e))
    };
    Ok(input_int-1)
}

fn computer_move(_board: &Board) -> u8 {
    // for now pick a random number from 0-6
    // we'll want to run minmax here...
    let mut rng = thread_rng();
    rng.gen_range(0..=6)
}