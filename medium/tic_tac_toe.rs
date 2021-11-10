//rust 1.30.0 

use std::io::{self, BufRead};

const BOARD_LEN: usize = 3;

fn solve(board: Vec<Vec<String>>) {
    let mut winners: Vec<&str> = Vec::with_capacity(2);

    'outer: for &player in ["X", "O"].iter() {
        // Horizontal lines
        if board.iter().any(|row| row.iter().filter(|&cell| cell == player).count() == BOARD_LEN) {
            winners.push(player);
            continue;
        }

        // Vertical lines
        for j in 0..BOARD_LEN {
            if board.iter().filter(|&row| row[j] == player).count() == BOARD_LEN {
                winners.push(player);
                continue 'outer;
            }
        }

        // Diagonal lines
        if (0..BOARD_LEN).filter(|&i| board[i][i] == player).count() == BOARD_LEN {
            winners.push(player);
            continue;
        }

        if (0..BOARD_LEN)
            .filter(|&i| board[i][BOARD_LEN - 1 - i] == player)
            .count()
            == BOARD_LEN
        {
            winners.push(player);
        }
    }

    match winners[..] {
        ["X"] => println!("Player 1"),
        ["O"] => println!("Player 2"),
        _ => println!("Draw")
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut board: Vec<Vec<String>> = Vec::with_capacity(BOARD_LEN);

    for _ in 0..BOARD_LEN {
        let line = lines.next().unwrap().expect("cannot read input");

        board.push(line.split_whitespace().map(|x| x.to_string()).collect());
    }

    solve(board);
}
