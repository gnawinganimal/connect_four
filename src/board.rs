
use crate::chip::Chip;

use std::fmt::Display;

pub struct Board {
    dim: (usize, usize),
    inner: Vec<Vec<Chip>>,
}

#[derive(Copy, Clone, Debug)]
pub enum Error {
    NoCol,
    FullCol
}

impl Board {
    pub fn new(dim: (usize, usize)) -> Self {
        Self {
            dim,
            inner: vec![vec![]; dim.0],
        }
    }

    pub fn x_0(&self) -> usize {
        self.dim.0
    }

    pub fn y_0(&self) -> usize {
        self.dim.1
    }

    pub fn col(&self, x: usize) -> Option<&[Chip]> {
        Some(self.inner.get(x)?.as_slice())
    }

    pub fn get(&self, (x, y): (usize, usize)) -> Option<Chip> {
        self.inner.get(x)?.get(y).copied()
    }

    pub fn plop(&mut self, x: usize, chip: Chip) -> Result<(usize, usize), Error> {
        if x >= self.x_0() {
            return Err(Error::NoCol);
        }

        if self.inner.get(x).unwrap().len() >= self.y_0() {
            return Err(Error::FullCol);
        }

        let pos = (x, self.inner.get(x).unwrap().len());
        self.inner.get_mut(x).unwrap().push(chip);
        Ok(pos)
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut s = String::new();

        for y in (0..self.y_0()).rev() {
            for x in 0..self.x_0() {
                s.push(match self.get((x, y)) {
                    Some(chip) => chip.into(),
                    None => '_',
                });
                s.push(' ');
            }
            s.push('\n');
        }

        write!(f, "{}", s)
    }
}
