use std::io;
use crate::board::Board;
use crate::coin::Coin;

pub fn read() -> io::Result<String> {
    let mut buffer = String::new();
    match io::stdin().read_line(&mut buffer) {
        Ok(_n) => Ok(buffer),
        Err(error) => Err(error),
    }
}

pub fn render_board(board: &Board) {
    for row_idx in (0..6).rev() {
        let mut line = String::from("|");
        for col_idx in 0..board.len() {
            match &board[col_idx][row_idx] {
                Coin::Player(player) => {
                    let s = format!("{}|", player);
                    line.push_str(&s);
                }, 
                Coin::Empty => line.push_str("x|"),
            }
        }
        println!("{}", line);
    }
    print!("\n");
}