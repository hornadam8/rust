use crate::card::{Card, Value};
use crate::deck::Deck;
use crate::util::{print_if, sleep};

#[derive(Debug, Clone)]
pub struct Hand {
    pub aces: u8,
    pub value: u8,
    pub cards: Vec<Card>,
}
impl Hand {
    pub fn default() -> Self {
        Self {
            cards: vec![],
            aces: 0,
            value: 0,
        }
    }

    pub fn new(deck: &mut Deck) -> Self {
        let mut hand = Hand {
            aces: 0,
            value: 0,
            cards: vec![deck.cards.remove(0), deck.cards.remove(0)],
        };
        hand.set_value();
        hand
    }

    pub fn is_blackjack(&self) -> bool {
        self.cards.len() == 2
            && ((self.cards[0].value == Value::Ace && self.cards[1].is_face_card())
                || (self.cards[1].value == Value::Ace && self.cards[0].is_face_card()))
    }

    pub fn hit(&mut self, deck: &mut Deck) {
        let card = deck.cards.remove(0);
        print_if("\n\nDrawing a card...");
        sleep(3);
        print_if(format!("\nDrew: {}", card.to_string()).as_str());
        sleep(2);
        self.cards.push(card);
        self.set_value();
        self.print();
        sleep(2);
    }

    pub fn set_value(&mut self) {
        self.value = 0;
        self.aces = 0;
        for card in self.cards.iter() {
            if let Value::Ace = card.value {
                self.aces += 1;
            }
            self.value += card.value.numeric_val();
        }
        for _ in 1..=self.aces {
            if self.value > 21 {
                self.value -= 10;
                self.aces -= 1;
            }
        }
        if self.value > 21 {
            self.value = 0;
        }
    }

    pub fn print(&self) {
        if self.value == 0 {
            print_if("Busted!");
            return;
        }
        if self.aces > 0 {
            print_if(
                format!(
                    "{}/{} | {}",
                    self.value,
                    self.value - 10,
                    self.card_string(),
                )
                .as_str(),
            );
        } else {
            print_if(format!("{} | {}\n\n", self.value, self.card_string()).as_str())
        }
    }

    pub fn card_string(&self) -> String {
        let mut output = String::new();
        for card in self.cards.iter() {
            output += (card.to_string() + " ").as_str();
        }
        output
    }

    pub fn get_count(&self) -> i8 {
        let mut count = 0;
        for card in self.cards.iter() {
            match card.value.numeric_val() {
                2 | 3 | 4 | 5 | 6 => count += 1,
                10 | 11 => count -= 1,
                _ => {}
            }
        }
        count
    }

    pub fn get_type(&self) -> HandType {
        if self.cards.len() > 2 {
            return HandType::ValueBased;
        }
        if self.is_ten_ten() {
            return HandType::TenTen;
        }
        if self.is_nine_nine() {
            return HandType::NineNine;
        }
        if self.is_eight_eight() {
            return HandType::EightEight;
        }
        if self.is_seven_seven() {
            return HandType::SevenSeven;
        }
        if self.is_six_six() {
            return HandType::SixSix;
        }
        if self.is_five_five() {
            return HandType::FiveFive;
        }
        if self.is_four_four() {
            return HandType::FourFour;
        }
        if self.is_three_three() {
            return HandType::ThreeThree;
        }
        if self.is_two_two() {
            return HandType::TwoTwo;
        }
        if self.is_ace_ace() {
            return HandType::AceAce;
        }
        if self.is_ace_ten() {
            return HandType::AceTen;
        }
        if self.is_ace_nine() {
            return HandType::AceNine;
        }
        if self.is_ace_eight() {
            return HandType::AceEight;
        }
        if self.is_ace_seven() {
            return HandType::AceSeven;
        }
        if self.is_ace_six() {
            return HandType::AceSix;
        }
        if self.is_ace_five() {
            return HandType::AceFive;
        }
        if self.is_ace_four() {
            return HandType::AceFour;
        }
        if self.is_ace_three() {
            return HandType::AceThree;
        }
        if self.is_ace_two() {
            return HandType::AceTwo;
        }

        return HandType::ValueBased;
    }

    fn is_ten_ten(&self) -> bool {
        self.cards.len() == 2
            && ((self.cards[0].value == Value::Ten && self.cards[1].value == Value::Ten)
                || (self.cards[1].value == Value::Jack && self.cards[0].value == Value::Jack)
                || (self.cards[1].value == Value::Queen && self.cards[0].value == Value::Queen)
                || (self.cards[1].value == Value::King && self.cards[0].value == Value::King))
    }

    fn is_nine_nine(&self) -> bool {
        self.cards.len() == 2
            && (self.cards[0].value == Value::Nine && self.cards[1].value == Value::Nine)
    }

    fn is_eight_eight(&self) -> bool {
        self.cards.len() == 2
            && (self.cards[0].value == Value::Eight && self.cards[1].value == Value::Eight)
    }

    fn is_seven_seven(&self) -> bool {
        self.cards.len() == 2
            && (self.cards[0].value == Value::Seven && self.cards[1].value == Value::Seven)
    }

    fn is_six_six(&self) -> bool {
        self.cards.len() == 2
            && (self.cards[0].value == Value::Six && self.cards[1].value == Value::Six)
    }

    fn is_five_five(&self) -> bool {
        self.cards.len() == 2
            && (self.cards[0].value == Value::Five && self.cards[1].value == Value::Five)
    }

    fn is_four_four(&self) -> bool {
        self.cards.len() == 2
            && (self.cards[0].value == Value::Four && self.cards[1].value == Value::Four)
    }

    fn is_three_three(&self) -> bool {
        self.cards.len() == 2
            && (self.cards[0].value == Value::Three && self.cards[1].value == Value::Three)
    }

    fn is_two_two(&self) -> bool {
        self.cards.len() == 2
            && (self.cards[0].value == Value::Two && self.cards[1].value == Value::Two)
    }

    fn is_ace_ace(&self) -> bool {
        self.cards.len() == 2
            && (self.cards[0].value == Value::Ace && self.cards[1].value == Value::Ace)
    }

    fn is_ace_ten(&self) -> bool {
        self.cards.len() == 2
            && ((self.cards[0].value == Value::Ten && self.cards[1].value == Value::Ace)
                || (self.cards[1].value == Value::Ace && self.cards[0].value == Value::Ten))
    }

    fn is_ace_nine(&self) -> bool {
        self.cards.len() == 2
            && ((self.cards[0].value == Value::Nine && self.cards[1].value == Value::Ace)
                || (self.cards[1].value == Value::Ace && self.cards[0].value == Value::Nine))
    }

    fn is_ace_eight(&self) -> bool {
        self.cards.len() == 2
            && ((self.cards[0].value == Value::Eight && self.cards[1].value == Value::Ace)
                || (self.cards[1].value == Value::Ace && self.cards[0].value == Value::Eight))
    }

    fn is_ace_seven(&self) -> bool {
        self.cards.len() == 2
            && ((self.cards[0].value == Value::Seven && self.cards[1].value == Value::Ace)
                || (self.cards[1].value == Value::Ace && self.cards[0].value == Value::Seven))
    }

    fn is_ace_six(&self) -> bool {
        self.cards.len() == 2
            && ((self.cards[0].value == Value::Six && self.cards[1].value == Value::Ace)
                || (self.cards[1].value == Value::Ace && self.cards[0].value == Value::Six))
    }

    fn is_ace_five(&self) -> bool {
        self.cards.len() == 2
            && ((self.cards[0].value == Value::Five && self.cards[1].value == Value::Ace)
                || (self.cards[1].value == Value::Ace && self.cards[0].value == Value::Five))
    }

    fn is_ace_four(&self) -> bool {
        self.cards.len() == 2
            && ((self.cards[0].value == Value::Four && self.cards[1].value == Value::Ace)
                || (self.cards[1].value == Value::Ace && self.cards[0].value == Value::Four))
    }

    fn is_ace_three(&self) -> bool {
        self.cards.len() == 2
            && ((self.cards[0].value == Value::Three && self.cards[1].value == Value::Ace)
                || (self.cards[1].value == Value::Ace && self.cards[0].value == Value::Three))
    }

    fn is_ace_two(&self) -> bool {
        self.cards.len() == 2
            && ((self.cards[0].value == Value::Two && self.cards[1].value == Value::Ace)
                || (self.cards[1].value == Value::Ace && self.cards[0].value == Value::Two))
    }
}

pub enum HandType {
    TenTen,
    NineNine,
    EightEight,
    SevenSeven,
    SixSix,
    FiveFive,
    FourFour,
    ThreeThree,
    TwoTwo,
    AceAce,
    AceTen,
    AceNine,
    AceEight,
    AceSeven,
    AceSix,
    AceFive,
    AceFour,
    AceThree,
    AceTwo,
    ValueBased,
}
