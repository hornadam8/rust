use crate::card::Value;
use crate::deck::Deck;
use crate::hand::Hand;
use crate::util::{print_if, sleep};

pub struct Dealer {
    pub hand: Hand,
}
impl Dealer {
    pub fn new() -> Self {
        Self {
            hand: Hand::default(),
        }
    }

    pub fn print_first(&self) {
        print_if(format!("Dealer showing {}\n\n", self.hand.cards[0].to_string()).as_str());
    }

    pub fn first_card_val(&self) -> Value {
        self.hand.cards[0].value.clone()
    }

    pub fn turn(&mut self, deck: &mut Deck) {
        print_if("Dealer turns");
        sleep(3);
        self.hand.print();
        while self.hand.value > 0 && (self.hand.value - self.hand.aces) < 17 {
            self.hand.hit(deck);
        }
    }
}
