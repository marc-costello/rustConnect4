use crate::coin::Coin;

pub const X_BOARD_LENGTH: usize = 7;
pub const Y_BOARD_LENGTH: usize = 6;

pub type Board = [[Coin; Y_BOARD_LENGTH]; X_BOARD_LENGTH];

pub fn new_board() -> Board {
    [[Coin::Empty; Y_BOARD_LENGTH]; X_BOARD_LENGTH]
}

pub fn drop_coin(board: &mut Board, coin: Coin, col: usize) -> (bool, (usize, usize)) {
    let col_idx = col;
    for i in 0..(&board[col_idx]).len() {
        if board[col_idx][i] == Coin::Empty {
            board[col_idx][i] = coin;
            return (true, (col_idx, i))
        }
    }
    (false, (0,0))
}
