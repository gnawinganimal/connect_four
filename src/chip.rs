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

impl From<Chip> for char {
    fn from(value: Chip) -> Self {
        match value {
            Chip::A => 'A',
            Chip::B => 'B',
        }
    }
}

impl Display for Chip {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", char::from(*self))
    }
}
