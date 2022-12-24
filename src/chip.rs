use std::fmt::Display;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Chip {
    A, B,
}

impl Chip {
    pub fn other(&self) -> Chip {
        match self {
            Self::A => Self::B,
            Self::B => Self::A,
        }
    }
}

impl Display for Chip {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::A => 'A',
            Self::B => 'B',
        })
    }
}
