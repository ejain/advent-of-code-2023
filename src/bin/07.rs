use std::fs::read_to_string;
use std::str::FromStr;
use counter::Counter;

fn main() {
    let input = read_to_string("data/07.txt").unwrap();
    let hands = Hand::parse_all(&input);
    println!("Part 1: {}", solve_part_1(&hands));
    println!("Part 2: {}", solve_part_2(&hands));
}

fn solve_part_1(hands: &[Hand]) -> u32 {
    let mut winnings = 0;
    for (i, hand) in Hand::sort(hands).iter().enumerate() {
        winnings += hand.bid * (i as u32 + 1);
    }
    winnings
}

fn solve_part_2(hands: &[Hand]) -> u32 {
    let hands: Vec<Hand> = hands.iter()
         .map(|hand| hand.replace(Card::Jack, Card::Joker))
         .collect();
    let to = &[
        Card::Ace, Card::King, Card::Queen, Card::Ten,
        Card::Nine, Card::Eight, Card::Seven, Card::Six,
        Card::Five, Card::Four, Card::Three, Card::Two,
    ];
    let mut best_hands = Vec::<Hand>::new();
    for hand in hands {
        let candidate_hands = hand.expand(Card::Joker, to);
        let sorted_hands = Hand::sort(&candidate_hands);
        let best_hand = sorted_hands.last().unwrap();
        best_hands.push(best_hand.clone());
    }
    solve_part_1(&best_hands)
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Card {
    Ace, King, Queen, Jack,
    Ten, Nine, Eight, Seven, Six, Five, Four, Three, Two,
    Joker
}

impl Card {
    fn parse_all(s: &str) -> Vec<Card> {
        s.chars()
            .map(|c| c.to_string().parse().unwrap())
            .collect()
    }
}

impl FromStr for Card {
    type Err = ();

    fn from_str(s: &str) -> Result<Card, Self::Err> {
        match s {
            "A" => Ok(Card::Ace),
            "K" => Ok(Card::King),
            "Q" => Ok(Card::Queen),
            "J" => Ok(Card::Jack),
            "T" => Ok(Card::Ten),
            "9" => Ok(Card::Nine),
            "8" => Ok(Card::Eight),
            "7" => Ok(Card::Seven),
            "6" => Ok(Card::Six),
            "5" => Ok(Card::Five),
            "4" => Ok(Card::Four),
            "3" => Ok(Card::Three),
            "2" => Ok(Card::Two),
            "*" => Ok(Card::Joker),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq)]
enum Type {
    FiveOfKind,
    FourOfKind,
    FullHouse,
    ThreeOfKind,
    TwoPair,
    OnePair,
    HighCard
}

#[derive(Clone, Debug, PartialEq)]
struct Hand {
    cards: Vec<Card>,
    original_cards: Vec<Card>,
    bid: u32,
}

impl Hand {
    fn parse_all(input: &str) -> Vec<Hand> {
        let mut hands = Vec::new();
        for line in input.lines().map(str::trim).filter(|line| !line.is_empty()) {
            hands.push(line.split_once(' ')
                .map(|(cards, bet)| Hand::new(Card::parse_all(cards), bet.parse().unwrap()))
                .unwrap()
            );
        }
        hands
    }

    fn sort(hands: &[Hand]) -> Vec<Hand> {
        let mut hands = hands.to_vec();
        hands.sort_by(|left, right| {
            let left_type = left.get_type() as u32;
            let right_type = right.get_type() as u32;
            right_type.cmp(&left_type)
                .then(right.original_cards.cmp(&left.original_cards))
                .then(right.cards.cmp(&left.cards))
        });
        hands
    }

    fn new(cards: Vec<Card>, bid: u32) -> Hand {
        Hand { cards: cards.to_vec(), original_cards: cards, bid }
    }

    fn get_type(&self) -> Type {
        let cards =  self.cards.iter().collect::<Counter<_>>();
        let most_common = cards.k_most_common_ordered(1);
        let (_, most_common_count) = most_common.first().unwrap();
        let most_common_count = *most_common_count as u32;
        match cards.len() {
            1 => Type::FiveOfKind,
            2 if most_common_count == 4 => Type::FourOfKind,
            2 if most_common_count == 3 => Type::FullHouse,
            3 if most_common_count == 3 => Type::ThreeOfKind,
            3 => Type::TwoPair,
            4 => Type::OnePair,
            5 => Type::HighCard,
            _ => panic!("bad hand of cards '{:?}'", self.cards)
        }
    }

    fn replace(&self, from: Card, to: Card) -> Hand {
        let cards = self.cards.iter()
            .map(|card| if *card == from { to } else { *card })
            .collect();
        Hand::new(cards, self.bid)
    }

    fn expand(&self, from: Card, to: &[Card]) -> Vec<Hand> {
        if !self.cards.contains(&from) {
            return vec![self.clone()]
        }
        let mut hands = Vec::new();
        let wildcard_index = self.cards.iter().position(|card| *card == from).unwrap();
        for replacement_card in to {
            let cards = self.cards.iter().enumerate()
                .map(|(i, card)| if i == wildcard_index { *replacement_card } else { *card })
                .collect();
            let hand = Hand { cards, original_cards: self.original_cards.to_vec(), bid: self.bid };
            hands.extend(hand.expand(from, to));
        }
        hands
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn setup() -> Vec<Hand> {
        Hand::parse_all("
            32T3K 765
            T55J5 684
            KK677 28
            KTJJT 220
            QQQJA 483
        ")
    }

    #[test]
    fn test_parse_all() {
        let hands = setup();
        assert_eq!(hands.len(), 5);
        assert_eq!(hands[0].cards, vec![
            Card::Three,
            Card::Two,
            Card::Ten,
            Card::Three,
            Card::King,
        ]);
        assert_eq!(hands[0].bid, 765);
    }

    #[test]
    fn test_get_type() {
        assert_type_eq("AAAAA", Type::FiveOfKind);
        assert_type_eq("AA8AA", Type::FourOfKind);
        assert_type_eq("23332", Type::FullHouse);
        assert_type_eq("TTT98", Type::ThreeOfKind);
        assert_type_eq("23432", Type::TwoPair);
        assert_type_eq("A23A4", Type::OnePair);
        assert_type_eq("23456", Type::HighCard);
    }

    fn assert_type_eq(cards: &str, expected_type: Type) {
        let hand = Hand::new(Card::parse_all(&cards), 0);
        assert_eq!(hand.get_type(), expected_type, "type of {:?}", &cards);
    }

    #[test]
    fn test_solve_part_1() {
        let hands = setup();
        assert_eq!(solve_part_1(&hands), 6440);
    }

    #[test]
    fn test_replace() {
        let hand = Hand::new(vec![
            Card::Jack,
            Card::King,
            Card::Jack,
        ], 42);
        assert_eq!(hand.replace(Card::Jack, Card::Joker).cards, vec![
            Card::Joker,
            Card::King,
            Card::Joker,
        ]);
    }

    #[test]
    fn test_expand() {
        let hand = Hand::new(vec![
            Card::Joker,
            Card::King,
            Card::Joker,
        ], 42);
        let hands = hand.expand(Card::Joker, &vec![Card::Two, Card::Three]);
        assert_eq!(hands.len(), 4);
        assert_eq!(hands[0].cards, vec![Card::Two, Card::King, Card::Two]);
        assert_eq!(hands[1].cards, vec![Card::Two, Card::King, Card::Three]);
        assert_eq!(hands[2].cards, vec![Card::Three, Card::King, Card::Two]);
        assert_eq!(hands[3].cards, vec![Card::Three, Card::King, Card::Three]);
    }

    #[test]
    fn test_solve_part_2() {
        let hands = setup();
        assert_eq!(solve_part_2(&hands), 5905);
    }
}
