use super::get_winners;

type Ticket = (u32, Vec<i64>, Vec<i64>);

pub(crate) struct Day4p2<'a> {
    cards: &'a [Ticket],
    idx: usize,
    copies: Vec<usize>,
    winners: Vec<Option<usize>>,
}

impl<'a> Day4p2<'a> {
    pub fn new(cards: &'a [Ticket]) -> Self {
        Self {
            cards,
            idx: 0,
            copies: vec![1; cards.len()],
            winners: vec![None; cards.len()],
        }
    }

    fn get_winners(&mut self) -> usize {
        if let Some(i) = self.winners[self.idx] {
            i
        } else {
            let wins = get_winners(&self.cards[self.idx]);
            self.winners[self.idx] = Some(wins);
            wins
        }
    }
}

impl<'a> Iterator for Day4p2<'a> {
    type Item = (usize, Ticket);

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.cards.len() {
            return None;
        }

        let wins = self.get_winners();
        for i in 1..=wins {
            self.copies[self.idx + i] += self.copies[self.idx];
        }

        let next = (self.copies[self.idx], self.cards[self.idx].clone());
        self.idx += 1;
        Some(next)
    }
}
