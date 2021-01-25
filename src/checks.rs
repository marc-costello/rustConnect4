use std::rc::Rc;
use crate::board::Board;
use crate::player::{Player};

// fn is_horizontal_win(grid: &Grid) -> Option<Player> {

// }

fn is_vertical_win(board: &Board, player: Rc<Player>) -> Option<u8> {
    // for col in board.iter() {
    //     col.iter().fold(0, |acc, coin| {
    //         match coin {
    //             Some(m) => {
                    
    //             }, 
    //             None => 0
    //         }
    //     });
    // }
    None
}

// fn is_diagonial_win(grid: &Grid) -> Option<Player> {

// }


#[cfg(test)]
mod tests {
    use crate::board::*;
    use crate::r#move::Move;
    use crate::player::{Player, PlayerType};
    use std::rc::Rc;

    const HUMAN_PLAYER: Player = Player {
        player_char: 'R',
        player_type: PlayerType::Human
    };

    #[test]
    fn vertical_win_test() {
        let mut board = new_board();
        let human_player = Rc::new(HUMAN_PLAYER);
        for _ in 0..4 {
            apply_move(&mut board, Move::new(human_player.clone(), 0));
        }

        let winning_col = super::is_vertical_win(&board, human_player.clone()).unwrap();

        //assert_eq!(*winning_player, *human_player.as_ref());
    }
}