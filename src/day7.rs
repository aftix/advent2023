use std::cmp::{PartialOrd, Reverse};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct Card(u8);

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            'A' => Self(14),
            'K' => Self(13),
            'Q' => Self(12),
            'J' => Self(11),
            'T' => Self(10),
            ch if ch.is_ascii_digit() && ch != '1' => Self(ch.as_ascii().unwrap().to_u8() - 48),
            _ => Self(2),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum HandType {
    HighCard(Card, Card, Card, Card, Card),
    Pair(Card, Card, Card, Card),   // pair, remaining...
    TwoPair(Card, Card, Card),      // pair, pair, remaining
    ThreeOfAKind(Card, Card, Card), // Card with 3, 1, 1
    FullHouse(Card, Card),          // card with 3, card with 2
    FourOfAKind(Card, Card),        // card with 4, card with 1
    FiveOfAKind(Card),
}

impl TryFrom<&str> for HandType {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() != 5 {
            return Err(());
        }

        let predicate = |ch: char| "AKQJT98765432".contains(ch);
        if !value.chars().all(predicate) {
            return Err(());
        }

        let mut cards: Vec<Card> = value.chars().map(Into::into).collect();
        cards.sort_by_key(|&c| Reverse(c));
        let (mut deduped, lastcard, amount) = cards.into_iter().fold(
            (vec![], Card(0), 0),
            |(mut vec, lastcard, mut amount), card| {
                if card != lastcard && amount != 0 {
                    vec.push((lastcard, amount));
                    amount = 0;
                }
                (vec, card, amount + 1)
            },
        );
        if amount != 0 {
            deduped.push((lastcard, amount));
        }
        deduped.sort_by_key(|&(_, amount)| Reverse(amount));

        let top = deduped[0];
        match top.1 {
            5 => Ok(Self::FiveOfAKind(top.0.into())),
            4 => Ok(Self::FourOfAKind(top.0.into(), deduped[1].0.into())),
            3 => {
                let next = deduped[1];
                match next.1 {
                    2 => Ok(Self::FullHouse(top.0.into(), next.0.into())),
                    _ => Ok(Self::ThreeOfAKind(
                        top.0.into(),
                        next.0.into(),
                        deduped[2].0.into(),
                    )),
                }
            }
            2 => {
                let next = deduped[1];
                match next.1 {
                    2 => Ok(Self::TwoPair(
                        top.0.into(),
                        next.0.into(),
                        deduped[2].0.into(),
                    )),
                    _ => Ok(Self::Pair(
                        top.0.into(),
                        next.0.into(),
                        deduped[2].0.into(),
                        deduped[3].0.into(),
                    )),
                }
            }
            _ => {
                let cards: Vec<Card> = deduped.iter().copied().map(|(ch, _)| ch.into()).collect();
                Ok(Self::HighCard(
                    cards[0], cards[1], cards[2], cards[3], cards[4],
                ))
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Card, HandType};
    use test_case::test_case;

    const CA: Card = Card(14);
    const CK: Card = Card(13);
    const CQ: Card = Card(12);
    const CJ: Card = Card(11);
    const CT: Card = Card(10);
    const C9: Card = Card(9);
    const C8: Card = Card(8);
    const C7: Card = Card(7);
    const C6: Card = Card(6);
    const C5: Card = Card(5);
    const C4: Card = Card(4);
    const C3: Card = Card(3);
    const C2: Card = Card(2);

    #[test_case("32T3K" => HandType::Pair(C3, CK, CT, C2))]
    #[test_case("T55J5" => HandType::ThreeOfAKind(C5, CJ, CT))]
    #[test_case("KK677" => HandType::TwoPair(CK, C7, C6))]
    #[test_case("KTJJT" => HandType::TwoPair(CJ, CT, CK))]
    #[test_case("QQQJA" => HandType::ThreeOfAKind(CQ, CA, CJ))]
    #[test_case("94J8A" => HandType::HighCard(CA, CJ, C9, C8, C4))]
    #[test_case("JK59A" => HandType::HighCard(CA, CK, CJ, C9, C5))]
    #[test_case("Q5QQQ" => HandType::FourOfAKind(CQ, C5))]
    #[test_case("T99T2" => HandType::TwoPair(CT, C9, C2))]
    #[test_case("595JQ" => HandType::Pair(C5, CQ, CJ, C9))]
    #[test_case("98299" => HandType::ThreeOfAKind(C9, C8, C2))]
    #[test_case("T596T" => HandType::Pair(CT, C9, C6, C5))]
    #[test_case("JQ999" => HandType::ThreeOfAKind(C9, CQ, CJ))]
    #[test_case("J3K39" => HandType::Pair(C3, CK, CJ, C9))]
    #[test_case("2999T" => HandType::ThreeOfAKind(C9, CT, C2))]
    #[test_case("KQ4K4" => HandType::TwoPair(CK, C4, CQ))]
    #[test_case("TT6TT" => HandType::FourOfAKind(CT, C6))]
    #[test_case("QAQJQ" => HandType::ThreeOfAKind(CQ, CA, CJ))]
    #[test_case("T87J4" => HandType::HighCard(CJ, CT, C8, C7, C4))]
    #[test_case("72272" => HandType::FullHouse(C2, C7))]
    #[test_case("67767" => HandType::FullHouse(C7, C6))]
    #[test_case("Q9A52" => HandType::HighCard(CA, CQ, C9, C5, C2))]
    #[test_case("Q7QK7" => HandType::TwoPair(CQ, C7, CK))]
    #[test_case("T63K8" => HandType::HighCard(CK, CT, C8, C6, C3))]
    #[test_case("TKKKK" => HandType::FourOfAKind(CK, CT))]
    #[test_case("6JJ66" => HandType::FullHouse(C6, CJ))]
    fn try_from(inp: &str) -> HandType {
        inp.try_into().unwrap()
    }

    #[test]
    fn sort() {
        use rand::{seq::SliceRandom, thread_rng};

        let sorted = vec![
            HandType::HighCard(CJ, CT, C8, C7, C4),
            HandType::HighCard(CK, CT, C8, C6, C3),
            HandType::HighCard(CA, CJ, C9, C8, C4),
            HandType::HighCard(CA, CQ, C9, C5, C2),
            HandType::HighCard(CA, CK, CJ, C9, C5),
            HandType::Pair(C3, CK, CT, C2),
            HandType::Pair(C3, CK, CJ, C9),
            HandType::Pair(C5, CQ, CJ, C9),
            HandType::Pair(CT, C9, C6, C5),
            HandType::TwoPair(CT, C9, C2),
            HandType::TwoPair(CJ, CT, CK),
            HandType::TwoPair(CQ, C7, CK),
            HandType::TwoPair(CK, C4, CQ),
            HandType::TwoPair(CK, C7, C6),
            HandType::ThreeOfAKind(C5, CJ, CT),
            HandType::ThreeOfAKind(C9, C8, C2),
            HandType::ThreeOfAKind(C9, CT, C2),
            HandType::ThreeOfAKind(C9, CQ, CJ),
            HandType::ThreeOfAKind(CQ, CA, CJ),
            HandType::ThreeOfAKind(CQ, CA, CJ),
            HandType::FullHouse(C2, C7),
            HandType::FullHouse(C6, CJ),
            HandType::FullHouse(C7, C6),
            HandType::FourOfAKind(CT, C6),
            HandType::FourOfAKind(CQ, C5),
            HandType::FourOfAKind(CK, CT),
        ];

        let mut rng = thread_rng();
        let mut unsorted = sorted.clone();
        unsorted.shuffle(&mut rng);
        unsorted.sort();
        assert_eq!(sorted, unsorted);
    }

    #[test_case("" ; "when empty")]
    #[test_case("A" ; "A")]
    #[test_case("AA" ; "AA")]
    #[test_case("AAA" ; "AAA")]
    #[test_case("AAAA" ; "AAAA")]
    #[test_case("AAAAAA" ; "AAAAAA")]
    #[test_case("AAAAZ" ; "AAAAZ")]
    #[test_case("ZAAAA" ; "ZAAAA")]
    #[should_panic]
    fn try_from_panics(inp: &str) {
        TryInto::<HandType>::try_into(inp).unwrap();
    }
}
