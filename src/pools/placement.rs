use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub enum Placement {
    Absolute(usize),
    Tied(usize),
}

impl Placement {
    pub fn inner(&self) -> usize {
        match self {
            Placement::Absolute(inner) => *inner,
            Placement::Tied(inner) => *inner,
        }
    }

    pub fn to_tied(&mut self) {
        *self = Placement::Tied(self.inner());
    }
}

impl Display for Placement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Absolute(place) => write!(f, "{}", place),
            Self::Tied(place) => write!(f, "{}T", place),
        }
    }
}
