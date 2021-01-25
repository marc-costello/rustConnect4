use std::io;
use crate::board::Board;

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
                Some(m) => {
                    let s = format!("{}|", &m.player().player_char);
                    line.push_str(&s);
                }, 
                None => line.push_str("x|"),
            }
        }
        println!("{}", line);
    }
    print!("\n");
}