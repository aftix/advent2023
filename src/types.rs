#[derive(Default, Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct GameSet {
    pub red: i64,
    pub green: i64,
    pub blue: i64,
}

#[derive(Default, Debug, PartialEq, Eq, Clone, Hash)]
pub struct Game {
    pub id: i64,
    pub sets: Vec<GameSet>,
}

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Span(pub usize, pub usize);

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Schematic {
    Number(i64, Span),
    Symbol(char, Span),
}

impl Schematic {
    pub fn is_number(&self) -> bool {
        match self {
            Self::Number(_, _) => true,
            Self::Symbol(_, _) => false,
        }
    }

    pub fn is_symbol(&self) -> bool {
        match self {
            Self::Number(_, _) => false,
            Self::Symbol(_, _) => true,
        }
    }

    pub fn span(&self) -> Span {
        match self {
            Self::Number(_, span) | Self::Symbol(_, span) => *span,
        }
    }
}
