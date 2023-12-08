#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
pub struct GameSet {
    pub red: i64,
    pub green: i64,
    pub blue: i64,
}

#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub struct Game {
    pub id: i64,
    pub sets: Vec<GameSet>,
}
