use crate::board::{Board, X_BOARD_LENGTH, Y_BOARD_LENGTH};
use crate::player::{Player};
use crate::coin::Coin;
use std::slice::Iter;

fn check_line_for_win(line_iter: Iter<Coin>, player: &Player) -> bool {
    let hits = line_iter.fold(0, |acc: u8, coin| {
        if acc >= 4 {
            return acc
        }
        match coin {
            Coin::Player(p) if *p == *player => acc + 1,
            Coin::Player(_) | Coin::Empty => 0,
        }
    });
    hits >= 4
}

pub fn is_vertical_win(board: &Board, player: Player) -> Option<u8> {
    for (col_idx, col) in board.iter().enumerate() {
        let is_win = check_line_for_win(col.iter(), &player);
        if is_win {
            return Some(col_idx as u8)
        }
    }
    None
}

pub fn is_horizontal_win(board: &Board, player: Player) -> Option<u8> {
    // flatten the board first
    let flat_board = board
                    .into_iter()
                    .flatten()
                    .map(|s| *s)
                    .collect::<Vec<_>>();
    
    for i in 0..X_BOARD_LENGTH {
        // get the horizontal row
        let base_board = &flat_board[i..];
        let row = base_board.iter()
                            .enumerate()
                            .filter(|(i, _)| (*i % Y_BOARD_LENGTH) == 0 || *i == 0)
                            .map(|(_, coin)| *coin)
                            .collect::<Vec<_>>();
        
        let is_win = check_line_for_win(row.iter(), &player);
        if is_win {
            return Some(i as u8)
        }
    }
    None
}

pub fn is_diagonal_win(board: &Board, player: Player) -> Option<u8> {
    // create a vector to hold all possible line variations
    let mut lines: Vec<Vec<Coin>> = Vec::with_capacity(X_BOARD_LENGTH * 2);

    add_range_to_collection(Range::Forward(0..X_BOARD_LENGTH), &mut lines, board, |x, y| (x as isize + 1, y as isize + 1));
    add_range_to_collection(Range::Backward((0..X_BOARD_LENGTH).rev()), &mut lines, board, |x, y| (x as isize - 1, y as isize + 1));

    // now we can run checks for each line in lines...
    for (i, line) in lines.iter().enumerate() {
        if check_line_for_win(line.iter(), &player) {
            return Some(i as u8)
        }
    }

    None
}

fn add_range_to_collection<F>(range: Range, collection: &mut Vec<Vec<Coin>>, board: &Board, next: F)
    where F: Fn(usize, usize) -> (isize, isize) {
        let get_diagonal_line = |starting_idx: usize| {
            let mut line: Vec<Coin> = Vec::new();
            let mut x = starting_idx;
            let mut y = 0;
            while x < X_BOARD_LENGTH && y < Y_BOARD_LENGTH {
                line.push(board[x][y]);
                let (new_x, new_y) = next(x, y);
                if new_x < 0 || new_y < 0 {
                    break
                }
                x = new_x as usize;
                y = new_y as usize;
            }
            line
        };
    
        for col_idx in range {
            let line = get_diagonal_line(col_idx);
            if line.len() < 1 {
                continue
            }
            collection.push(line);
        }
}

pub fn is_draw(board: &Board) -> bool {
    !board.iter().flatten().any(|coin| *coin == Coin::Empty)
}

enum Range {
    Forward(std::ops::Range<usize>),
    Backward(std::iter::Rev<std::ops::Range<usize>>),
}
impl Iterator for Range {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        match self {
            Range::Forward(range) => range.next(),
            Range::Backward(range) => range.next(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::board::*;
    use crate::coin::Coin;
    use crate::player::Player;

    #[test]
    fn vertical_win_with_4() {
        let mut board = new_board();
        for _ in 0..4 {
            drop_coin(&mut board, Coin::Player(Player::Red), 0);
        }

        let winning_col = super::is_vertical_win(&board, Player::Red).unwrap();

        assert_eq!(winning_col, 0);
    }

    #[test]
    fn should_not_win_vertical_with_less_than_4() {
        let mut board = new_board();
        for _ in 0..3 {
            drop_coin(&mut board, Coin::Player(Player::Red), 0);
        }
        drop_coin(&mut board, Coin::Player(Player::Yellow), 0);
        for _ in 0..2 {
            drop_coin(&mut board, Coin::Player(Player::Red), 0);
        }

        let win_result = super::is_vertical_win(&board, Player::Red);

        assert_eq!(win_result, None);
    }

    #[test]
    fn horizontal_win_with_4() {
        let mut board = new_board();
        for i in 0..4 {
            drop_coin(&mut board, Coin::Player(Player::Red), i);
        }
        
        let winning_row = super::is_horizontal_win(&board, Player::Red).unwrap();

        assert_eq!(winning_row, 0);
    }

    #[test]
    fn should_not_win_horizontal_with_less_than_4() {
        let mut board = new_board();
        for i in 0..3 {
            drop_coin(&mut board, Coin::Player(Player::Red), i);
        }
        drop_coin(&mut board, Coin::Player(Player::Yellow), 3);
        
        let result = super::is_horizontal_win(&board, Player::Red);

        assert_eq!(result, None);
    }

    #[test]
    fn diagonal_win_forwards_with_4() {
        let mut board = new_board();
        for i in 0..4 {
            board[i][i] = Coin::Player(Player::Red);
        }
        super::is_diagonal_win(&board, Player::Red).unwrap();
    }

    #[test]
    fn diagonal_win_backwards_with_4() {
        let mut board = new_board();
        let mut current_coord = (4,0);
        while current_coord != (0,4) {
            let (x, y) = current_coord;
            board[x][y] = Coin::Player(Player::Red);
            current_coord = (x-1, y+1)
        }
        board.iter().for_each(|r| println!("{:?}", r));
        super::is_diagonal_win(&board, Player::Red).unwrap();
    }

    #[test]
    fn is_draw_is_true_when_a_board_has_no_empty_coins() {
        let mut board = new_board();
        for x in 0..X_BOARD_LENGTH {
            for y in 0..Y_BOARD_LENGTH {
                board[x][y] = Coin::Player(Player::Yellow);
            }
        }
        assert_eq!(super::is_draw(&board), true);
    }
}