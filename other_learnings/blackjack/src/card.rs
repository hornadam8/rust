#[derive(Debug, Clone)]
pub struct Card {
    pub value: Value,
    pub suit: Suit,
}

impl Card {
    pub fn to_string(&self) -> String {
        return format!(
            r#"
        ┌─────────┐
        │{}       │
        │         │
        │         │
        │    {}    │
        │         │
        │         │
        │       {}│
        └─────────┘
        "#,
            self.value.string_val(),
            self.suit.string_val(),
            self.value.string_val()
        );
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
impl Suit {
    pub fn string_val(&self) -> String {
        return match self {
            Suit::Clubs => '\u{2663}'.to_string(),
            Suit::Diamonds => '\u{2666}'.to_string(),
            Suit::Hearts => '\u{2665}'.to_string(),
            Suit::Spades => '\u{2660}'.to_string(),
        };
    }
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
    pub fn string_val(&self) -> String {
        return match self {
            Value::Ace => "A ".to_string(),
            Value::King => "K ".to_string(),
            Value::Queen => "Q ".to_string(),
            Value::Jack => "J ".to_string(),
            Value::Ten => "10".to_string(),
            Value::Nine => "9 ".to_string(),
            Value::Eight => "8 ".to_string(),
            Value::Seven => "7 ".to_string(),
            Value::Six => "6 ".to_string(),
            Value::Five => "5 ".to_string(),
            Value::Four => "4 ".to_string(),
            Value::Three => "3 ".to_string(),
            Value::Two => "2 ".to_string(),
        };
    }
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
