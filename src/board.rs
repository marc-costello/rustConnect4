use crate::r#move::Move;

pub type Board = [[Option<Move>; 6]; 7];

pub fn new_board() -> Board {
    Default::default()
}

pub fn apply_move(board: &mut Board, m: Move) -> bool {
    let col_idx = m.x;
    for i in 0..(&board[col_idx]).len() {
        if board[col_idx][i] == None {
            board[col_idx][i] = Some(m);
            return true
        }
    }
    false
}
