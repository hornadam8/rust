use crate::card::Value;
use crate::deck::Deck;
use crate::hand::HandType;
use crate::player::{Player, PlayerType};
use crate::util::{print_if, sleep};
use std::fmt::format;

impl Player {
    pub fn new_edgar(num: u8) -> Self {
        println!("Edgar Allen Poe has joined the game!");
        sleep(2);
        println!("\n\nEdgar A. Poe - \"I invented the term \'by the book\' you know?\"\n\n");
        sleep(2);
        if num == 0 {
            Self::new("Edgar A. Poe".to_string(), PlayerType::EdgarAI)
        } else {
            Self::new(format!("Edgar A. Poe({num})"), PlayerType::EdgarAI)
        }
    }

    pub fn edgar_turn(&mut self, deck: &mut Deck, dealer_showing: Value) {
        sleep(3);
        if self.hand.is_blackjack() {
            print_if("Blackjack! You win");
            return;
        }
        self.hand.print();
        while self.hand.value > 0 {
            match self.hand.get_type() {
                HandType::TenTen => {
                    print_if(format!("{} stood!", self.name).as_str());
                    return;
                }
                HandType::NineNine => {
                    if dealer_showing == Value::Seven
                        || dealer_showing == Value::Ten
                        || dealer_showing == Value::Jack
                        || dealer_showing == Value::Queen
                        || dealer_showing == Value::King
                        || dealer_showing == Value::Ace
                        || self.wager * 2.0 > self.funds
                    {
                        print_if(format!("{} stood!", self.name).as_str());
                        return;
                    } else {
                        self.split_hand(deck);
                    }
                }
                HandType::EightEight => self.split_hand(deck),
                HandType::SevenSeven => {
                    if dealer_showing == Value::Eight
                        || dealer_showing == Value::Nine
                        || dealer_showing == Value::Ten
                        || dealer_showing == Value::Jack
                        || dealer_showing == Value::Queen
                        || dealer_showing == Value::King
                        || dealer_showing == Value::Ace
                        || self.wager * 2.0 > self.funds
                    {
                        self.hit(deck, false);
                    } else {
                        self.split_hand(deck);
                    }
                }
                HandType::SixSix => {
                    if (dealer_showing == Value::Two
                        || dealer_showing == Value::Three
                        || dealer_showing == Value::Four
                        || dealer_showing == Value::Five
                        || dealer_showing == Value::Six)
                        && self.wager * 2.0 <= self.funds
                    {
                        self.split_hand(deck);
                    } else {
                        self.hit(deck, false);
                    }
                }
                HandType::FiveFive => {
                    if dealer_showing == Value::Ten
                        || dealer_showing == Value::Jack
                        || dealer_showing == Value::Queen
                        || dealer_showing == Value::King
                        || dealer_showing == Value::Ace
                        || self.wager * 2.0 > self.funds
                    {
                        self.hit(deck, false);
                    } else {
                        self.double_down(deck, false);
                        return;
                    }
                }
                HandType::FourFour => {
                    if (dealer_showing == Value::Five || dealer_showing == Value::Six)
                        && self.wager * 2.0 <= self.funds
                    {
                        self.split_hand(deck);
                    } else {
                        self.hit(deck, false);
                    }
                }
                HandType::ThreeThree => {
                    if (dealer_showing == Value::Two
                        || dealer_showing == Value::Three
                        || dealer_showing == Value::Four
                        || dealer_showing == Value::Five
                        || dealer_showing == Value::Six
                        || dealer_showing == Value::Seven)
                        && self.wager * 2.0 < self.funds
                    {
                        self.split_hand(deck);
                    } else {
                        self.hit(deck, false);
                    }
                }
                HandType::TwoTwo => {
                    if (dealer_showing == Value::Two
                        || dealer_showing == Value::Three
                        || dealer_showing == Value::Four
                        || dealer_showing == Value::Five
                        || dealer_showing == Value::Six
                        || dealer_showing == Value::Seven)
                        && self.wager * 2.0 < self.funds
                    {
                        self.split_hand(deck);
                    } else {
                        self.hit(deck, false);
                    }
                }
                HandType::AceAce => {
                    if self.wager * 2.0 <= self.funds {
                        self.split_hand(deck);
                    } else {
                        self.hit(deck, false);
                    }
                }
                HandType::AceTen => {
                    print_if(format!("{} stood!", self.name).as_str());
                    return;
                }
                HandType::AceNine => {
                    print_if(format!("{} stood!", self.name).as_str());
                    return;
                }
                HandType::AceEight => {
                    if dealer_showing == Value::Six && self.wager * 2.0 <= self.funds {
                        self.double_down(deck, false);
                        return;
                    } else {
                        print_if(format!("{} stood!", self.name).as_str());
                        return;
                    }
                }
                HandType::AceSeven => {
                    if (dealer_showing == Value::Two
                        || dealer_showing == Value::Three
                        || dealer_showing == Value::Four
                        || dealer_showing == Value::Five
                        || dealer_showing == Value::Six)
                        && self.wager * 2.0 <= self.funds
                    {
                        self.double_down(deck, false);
                        return;
                    } else if dealer_showing == Value::Nine
                        || dealer_showing == Value::Ten
                        || dealer_showing == Value::Jack
                        || dealer_showing == Value::Queen
                        || dealer_showing == Value::King
                        || dealer_showing == Value::Ace
                    {
                        self.hit(deck, false);
                    } else {
                        print_if(format!("{} stood!", self.name).as_str());
                        return;
                    }
                }
                HandType::AceSix => {
                    if (dealer_showing == Value::Three
                        || dealer_showing == Value::Four
                        || dealer_showing == Value::Five
                        || dealer_showing == Value::Six)
                        && self.wager * 2.0 <= self.funds
                    {
                        self.double_down(deck, false);
                        return;
                    } else {
                        self.hit(deck, false);
                    }
                }
                HandType::AceFive => {
                    if (dealer_showing == Value::Four
                        || dealer_showing == Value::Five
                        || dealer_showing == Value::Six)
                        && self.wager * 2.0 <= self.funds
                    {
                        self.double_down(deck, false);
                        return;
                    } else {
                        self.hit(deck, false);
                    }
                }
                HandType::AceFour => {
                    if (dealer_showing == Value::Four
                        || dealer_showing == Value::Five
                        || dealer_showing == Value::Six)
                        && self.wager * 2.0 <= self.funds
                    {
                        self.double_down(deck, false);
                        return;
                    } else {
                        self.hit(deck, false);
                    }
                }
                HandType::AceThree => {
                    if (dealer_showing == Value::Five || dealer_showing == Value::Six)
                        && self.wager * 2.0 <= self.funds
                    {
                        self.double_down(deck, false);
                        return;
                    } else {
                        self.hit(deck, false);
                    }
                }
                HandType::AceTwo => {
                    if (dealer_showing == Value::Five || dealer_showing == Value::Six)
                        && self.wager * 2.0 <= self.funds
                    {
                        self.double_down(deck, false);
                        return;
                    } else {
                        self.hit(deck, false);
                    }
                }
                HandType::ValueBased => {
                    if self.hand.value >= 17 {
                        print_if(format!("{} stood!", self.name).as_str());
                        return;
                    } else if self.hand.value <= 16 && self.hand.value >= 13 {
                        if dealer_showing == Value::Two
                            || dealer_showing == Value::Three
                            || dealer_showing == Value::Four
                            || dealer_showing == Value::Five
                            || dealer_showing == Value::Six
                        {
                            print_if(format!("{} stood!", self.name).as_str());
                            return;
                        } else {
                            self.hit(deck, false);
                        }
                    } else if self.hand.value == 12 {
                        if dealer_showing == Value::Four
                            || dealer_showing == Value::Five
                            || dealer_showing == Value::Six
                        {
                            print_if(format!("{} stood!", self.name).as_str());
                            return;
                        } else {
                            self.hit(deck, false);
                        }
                    } else if self.hand.value == 11 {
                        if self.hand.cards.len() == 2 && self.wager * 2.0 <= self.funds {
                            self.double_down(deck, false);
                            return;
                        } else {
                            self.hit(deck, false);
                        }
                    } else if self.hand.value == 10 {
                        if dealer_showing == Value::Ten
                            || dealer_showing == Value::Jack
                            || dealer_showing == Value::Queen
                            || dealer_showing == Value::King
                            || dealer_showing == Value::Ace
                            || self.wager * 2.0 > self.funds
                        {
                            self.hit(deck, false);
                        } else {
                            self.double_down(deck, false);
                            return;
                        }
                    } else if self.hand.value == 9 {
                        if dealer_showing == Value::Two
                            || dealer_showing == Value::Seven
                            || dealer_showing == Value::Eight
                            || dealer_showing == Value::Nine
                            || dealer_showing == Value::Ten
                            || dealer_showing == Value::Jack
                            || dealer_showing == Value::Queen
                            || dealer_showing == Value::King
                            || dealer_showing == Value::Ace
                            || self.wager * 2.0 > self.funds
                        {
                            self.hit(deck, false);
                        } else {
                            self.double_down(deck, false);
                            return;
                        }
                    } else if self.hand.value <= 8 {
                        self.hit(deck, false);
                    } else {
                        return;
                    }
                }
            }
        }

        match self.split_hand.as_mut() {
            Some(hand) => {
                if hand.is_blackjack() {
                    print_if(format!("Blackjack! {} wins", self.name).as_str());
                    return;
                }
                match hand.get_type() {
                    HandType::TenTen => {
                        print_if(format!("{} stood!", self.name).as_str());
                        return;
                    }
                    HandType::NineNine => {
                        return;
                    }
                    HandType::EightEight => {
                        if dealer_showing.numeric_val() <= 6 {
                            return;
                        } else {
                            self.hit(deck, true);
                        }
                    }
                    HandType::SevenSeven => {
                        if dealer_showing == Value::Seven
                            || dealer_showing == Value::Eight
                            || dealer_showing == Value::Nine
                            || dealer_showing == Value::Ten
                            || dealer_showing == Value::Jack
                            || dealer_showing == Value::Queen
                            || dealer_showing == Value::King
                            || dealer_showing == Value::Ace
                        {
                            self.hit(deck, true);
                        } else {
                            return;
                        }
                    }
                    HandType::SixSix => {
                        if dealer_showing == Value::Two
                            || dealer_showing == Value::Three
                            || dealer_showing == Value::Four
                            || dealer_showing == Value::Five
                            || dealer_showing == Value::Six
                        {
                            return;
                        } else {
                            self.hit(deck, true);
                        }
                    }
                    HandType::FiveFive => self.hit(deck, true),
                    HandType::FourFour => self.hit(deck, true),
                    HandType::ThreeThree => self.hit(deck, true),
                    HandType::TwoTwo => self.hit(deck, true),
                    HandType::AceAce => {
                        if dealer_showing == Value::Two
                            || dealer_showing == Value::Seven
                            || dealer_showing == Value::Eight
                            || dealer_showing == Value::Nine
                            || dealer_showing == Value::Ten
                            || dealer_showing == Value::Jack
                            || dealer_showing == Value::Queen
                            || dealer_showing == Value::King
                            || dealer_showing == Value::Ace
                        {
                            self.hit(deck, true)
                        } else {
                            return;
                        }
                    }
                    HandType::AceTen => {
                        print_if(format!("{} stood!", self.name).as_str());
                        return;
                    }
                    HandType::AceNine => {
                        print_if(format!("{} stood!", self.name).as_str());
                        return;
                    }
                    HandType::AceEight => {
                        print_if(format!("{} stood!", self.name).as_str());
                        return;
                    }
                    HandType::AceSeven => {
                        if dealer_showing == Value::Two
                            || dealer_showing == Value::Three
                            || dealer_showing == Value::Four
                            || dealer_showing == Value::Five
                            || dealer_showing == Value::Six
                            || dealer_showing == Value::Nine
                            || dealer_showing == Value::Ten
                            || dealer_showing == Value::Jack
                            || dealer_showing == Value::Queen
                            || dealer_showing == Value::King
                            || dealer_showing == Value::Ace
                        {
                            self.hit(deck, true);
                        } else {
                            print_if(format!("{} stood!", self.name).as_str());
                            return;
                        }
                    }
                    HandType::AceSix => self.hit(deck, true),
                    HandType::AceFive => self.hit(deck, true),
                    HandType::AceFour => self.hit(deck, true),
                    HandType::AceThree => self.hit(deck, true),
                    HandType::AceTwo => self.hit(deck, true),
                    HandType::ValueBased => {
                        if hand.value >= 17 {
                            return;
                        } else if hand.value <= 16 && hand.value >= 13 {
                            if dealer_showing == Value::Two
                                || dealer_showing == Value::Three
                                || dealer_showing == Value::Four
                                || dealer_showing == Value::Five
                                || dealer_showing == Value::Six
                            {
                                print_if(format!("{} stood!", self.name).as_str());
                                return;
                            } else {
                                self.hit(deck, true);
                            }
                        } else if hand.value == 12 {
                            if dealer_showing == Value::Four
                                || dealer_showing == Value::Five
                                || dealer_showing == Value::Six
                            {
                                return;
                            } else {
                                self.hit(deck, true);
                            }
                        } else if hand.value == 11 {
                            self.hit(deck, true)
                        } else if hand.value == 10 {
                            self.hit(deck, true)
                        } else if hand.value == 9 {
                            self.hit(deck, true)
                        } else if hand.value <= 8 {
                            self.hit(deck, true);
                        } else {
                            return;
                        }
                    }
                }
            }
            None => {}
        }
    }
}
