use std::rc::Rc;
use rand::{thread_rng, Rng};
use std::error::Error;
use std::io::{Error as ioError, ErrorKind};
use crate::player::{Player, PlayerType};
use crate::board::{self, Board};
use crate::r#move::Move;
use crate::renderer;

pub enum GameOutcome {
    Win(Rc<Player>),
    Draw
}

pub struct Game { 
    player_1: Rc<Player>,
    player_2: Rc<Player>,
    board: Board,
    current_player: Rc<Player>
 }

 impl Game {
     pub fn new_game() -> Game {
         let p1 = Rc::new(Player { player_type: PlayerType::Human, player_char: 'R' });
         let p2 = Rc::new(Player { player_type: PlayerType::Computer, player_char: 'Y' });
         
         Game {
             player_1: p1.clone(),
             player_2: p2.clone(),
             board: board::new_board(),
             current_player: p1.clone(),
         }
     }

     pub fn next_player(&mut self) -> &Self {
         self.current_player = if *self.current_player.as_ref() == *self.player_1.as_ref() {
             self.player_2.clone()
         } else {
             self.player_1.clone()
         };
         self
     }

     pub fn start(&mut self) {
        renderer::render_board(&self.board);
         
         // game loop 
         let outcome: Option<Rc<Player>> = loop {
             if *self.current_player.as_ref() == *self.player_1.as_ref() {
                match human_move(&self.board, self.player_1.clone()) {
                    Ok(m) => board::apply_move(&mut self.board, m),
                    Err(_) => continue,
                };
             } else {
                let computer_move = computer_move(&self.board, self.player_2.clone());
                board::apply_move(&mut self.board, computer_move);

                // render out the board with the new move
                renderer::render_board(&self.board);
             }
             
             // run win check.
             match self.check_for_winner() {
                 Some(GameOutcome::Win(player)) => break Some(player.clone()),
                 Some(GameOutcome::Draw) => break None,
                 None => self.next_player(),
             };
         };

         if let Some(player) = outcome {
            // announce winner
            println!("{:?} HAS WON!", player.as_ref().player_type);
         } else {
            // draw
            println!("THIS GAME IS A DRAW!");
         }
     }

     fn check_for_winner(&self) -> Option<GameOutcome> {
        None
     }
 }

fn human_move(_board: &Board, human_player: Rc<Player>) -> Result<Move, Box<dyn Error>> {
    let input = renderer::read()?;
    let input_int = match input.trim().parse::<u8>() {
        Ok(i) if i < 1 => 
            return Err(Box::new(ioError::new(ErrorKind::InvalidInput, "Input cannot be less than 1"))),
        Ok(i) => i,
        Err(e) => return Err(Box::new(e))
    };
    Ok(Move::new(human_player, input_int-1))
}


fn computer_move(_board: &Board, computer_player: Rc<Player>) -> Move {
    // for now pick a random number from 0-6
    // we'll want to run minmax here in future...
    let mut rng = thread_rng();
    let col: u8 = rng.gen_range(0..=6);
    
    Move::new(computer_player, col)
}