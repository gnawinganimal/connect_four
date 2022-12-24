
use crate::board::Board;
use crate::chip::Chip;

use std::cmp::{Ord, Ordering, min};

pub fn check(board: &Board, team: Chip, (x, y): (usize, usize)) -> bool {
    check_x(board, team, x) || check_y(board, team, y) || check_asc(board, team, (x, y)) || check_des(board, team, (x, y))
}

pub fn check_x(board: &Board, team: Chip, x: usize) -> bool {
    let mut count = 0;

    for y in 0..board.y() {
        if let Some(chip) = board.get((x, y)) {
            if team == chip {
                count += 1;

                if count == 4 {
                    return true;
                }

                continue;
            }
        }
        
        count = 0;
    };

    false
}

pub fn check_y(board: &Board, team: Chip, y: usize) -> bool {
    let mut count = 0;

    for x in 0..board.x() {
        if let Some(chip) = board.get((x, y)) {
            if team == chip {
                count += 1;

                if count == 4 {
                    return true;
                }

                continue;
            }
        }
        
        count = 0;
    };

    false
}

pub fn check_asc(board: &Board, team: Chip, (x, y): (usize, usize)) -> bool {
    // overshadow previous x and y 
    let (x, y) = match x.cmp(&y) {
        Ordering::Greater => (x - y, 0),
        Ordering::Less => (0, y - x),
        Ordering::Equal => (0, 0),
    };

    // calculate length of the diagonal
    let n = min(board.x() - x, board.y() - y);

    let mut count = 0;
    for b in 0..n {
        if let Some(chip) = board.get((x + b, y + b)) {
            if team == chip {
                count += 1;

                if count == 4 {
                    return true;
                }

                continue;
            }
        }
        
        count = 0;
    };

    false
}

pub fn check_des(board: &Board, team: Chip, (x, y): (usize, usize)) -> bool {
    let dy = board.y() - y;

    // overshadow previous x and y 
    let (x, y) = match x.cmp(&dy) {
        Ordering::Greater => (x - dy, board.y()),
        Ordering::Less => (0, y + x),
        Ordering::Equal => (0, board.y()),
    };

    // calculate length of the diagonal
    let n = min(board.x() - x, y) + 1;

    let mut count = 0;
    for b in 0..n {
        if let Some(chip) = board.get((x + b, y - b)) {
            if team == chip {
                count += 1;

                if count == 4 {
                    return true;
                }

                continue;
            }
        }
        
        count = 0;
    };

    false
}
