
pub mod board;
pub mod chip;
pub mod check;

pub use board::Board;
pub use chip::Chip;
pub use check::check;

use std::io;

fn main() -> io::Result<()> {
    let mut board = Board::new((7, 6));

    let mut team = Chip::A;
    'game: loop {
        // display board
        println!("{board}");

        // get input
        let x: usize = {
            println!("Please enter a column: ");
            let mut s = String::new();
            io::stdin().read_line(&mut s)?;

            match s.trim().parse::<usize>() {
                Ok(x) => x - 1,
                Err(_) => {
                    println!("ERROR: input must be a positive integer");
                    continue;
                },
            }
        };

        // plop chip and store position
        let last = match board.plop(x, team) {
            Ok(last) => last,
            Err(_) => {
                println!("ERROR: invalid chip placement");
                continue;
            },
        };

        // check for win
        if check(&board, team, last) {
            break 'game;
        }
        
        // switch team
        team = team.other();
    };

    println!("Team {team} won!");
    println!("{board}");

    Ok(())
}
