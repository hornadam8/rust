use crate::card::{Card, Suit, Value};
use crate::hand::Hand;
use rand::Rng;

const SUITS: [Suit; 4] = [Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades];
const VALUES: [Value; 13] = [
    Value::Ace,
    Value::King,
    Value::Queen,
    Value::Jack,
    Value::Ten,
    Value::Nine,
    Value::Eight,
    Value::Seven,
    Value::Six,
    Value::Five,
    Value::Four,
    Value::Three,
    Value::Two,
];

#[derive(Debug)]
pub struct Deck {
    pub cards: Vec<Card>,
}
impl Deck {
    pub fn new(count: u8) -> Self {
        let mut deck: Vec<Card> = vec![];
        for _ in 1..=count {
            for suit in SUITS {
                for value in VALUES {
                    deck.push(Card {
                        value,
                        suit: suit.clone(),
                    })
                }
            }
        }
        let mut deck = Deck { cards: deck };
        deck.shuffle();
        deck
    }

    pub fn shuffle(&mut self) {
        for _ in 1..=1_000_000 {
            let rand_card_i = rand::thread_rng().gen_range(0..(self.cards.len() - 1));
            let card = self.cards.remove(rand_card_i);
            self.cards.push(card);
        }
    }

    pub fn deal(&mut self) -> Hand {
        Hand::new(self)
    }
}
