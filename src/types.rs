use std::ops::Range;

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

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum Day5 {
    Seeds(Vec<i64>),
    MapTitle(String, String),
    Maps(Range<i64>, i64),
}

impl Day5 {
    pub fn seeds(self) -> Vec<i64> {
        if let Self::Seeds(vec) = self {
            vec
        } else {
            panic!("Called Day5::seeds on wrong enum variant");
        }
    }

    pub fn titles(self) -> (String, String) {
        if let Self::MapTitle(from, to) = self {
            (from, to)
        } else {
            panic!("Called Day5::titles on wrong enum variant");
        }
    }

    pub fn maps(self) -> Option<(Range<i64>, i64)> {
        if let Self::Maps(src, dest) = self {
            Some((src, dest))
        } else {
            None
        }
    }

    pub fn is_maps(&self) -> bool {
        matches!(self, Self::Maps(_, _))
    }
}
