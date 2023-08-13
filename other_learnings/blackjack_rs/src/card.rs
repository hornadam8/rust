#[derive(Debug, Clone)]
pub struct Card {
    pub value: Value,
    pub suit: Suit,
}

impl Card {
    pub fn to_string(&self) -> String {
        let mut output = String::new();
        match self.value {
            Value::Ace => output += "A",
            Value::King => output += "K",
            Value::Queen => output += "Q",
            Value::Jack => output += "J",
            Value::Ten => output += "10",
            Value::Nine => output += "9",
            Value::Eight => output += "8",
            Value::Seven => output += "7",
            Value::Six => output += "6",
            Value::Five => output += "5",
            Value::Four => output += "4",
            Value::Three => output += "3",
            Value::Two => output += "2",
        }
        match self.suit {
            Suit::Clubs => output += '\u{2663}'.to_string().as_str(),
            Suit::Diamonds => output += '\u{2666}'.to_string().as_str(),
            Suit::Hearts => output += '\u{2665}'.to_string().as_str(),
            Suit::Spades => output += '\u{2660}'.to_string().as_str(),
        }
        output
    }
    pub fn is_face_card(&self) -> bool {
        return match self.value {
            Value::Ace => false,
            Value::King => true,
            Value::Queen => true,
            Value::Jack => true,
            Value::Ten => false,
            Value::Nine => false,
            Value::Eight => false,
            Value::Seven => false,
            Value::Six => false,
            Value::Five => false,
            Value::Four => false,
            Value::Three => false,
            Value::Two => false,
        };
    }
}

#[derive(Debug, Clone)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Value {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}
impl Value {
    pub fn numeric_val(&self) -> u8 {
        return match self {
            Value::Ace => 11, // will get subtracted if necessary at hand value calculation
            Value::King => 10,
            Value::Queen => 10,
            Value::Jack => 10,
            Value::Ten => 10,
            Value::Nine => 9,
            Value::Eight => 8,
            Value::Seven => 7,
            Value::Six => 6,
            Value::Five => 5,
            Value::Four => 4,
            Value::Three => 3,
            Value::Two => 2,
        };
    }
}
