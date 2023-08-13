use crate::card::Value;
use crate::deck::Deck;
use crate::hand::Hand;
use crate::util::{print_if, sleep};
use rust_decimal::Decimal;

#[derive(Debug, Clone)]
pub struct Player {
    pub name: String,
    pub type_: PlayerType,
    pub hand: Hand,
    pub split_hand: Option<Hand>,
    pub funds: f64,
    pub wager: f64,
    pub stats: Stats,
    pub previous_wager: f64,
    pub previous_win: bool,
}
impl Player {
    pub fn new(name: String, player_type: PlayerType) -> Self {
        Player {
            name: name.clone(),
            type_: player_type,
            hand: Hand::default(),
            split_hand: None,
            funds: 10000.0,
            wager: 0.0,
            stats: Stats::default(name),
            previous_wager: 0.0,
            previous_win: true,
        }
    }

    pub fn print(&self) {
        print_if("\n-----------------");
        print_if(format!("{} | ${}", self.name, self.funds).as_str());
        print_if("-----------------");
    }

    pub fn print_w_wager(&self) {
        print_if("\n-------------------------");
        print_if(format!("{} | ${} | Wager: ${}", self.name, self.funds, self.wager).as_str());
        print_if("-------------------------");
    }

    pub fn hit(&mut self, deck: &mut Deck, split: bool) {
        print_if(format!("\n{} hit!", self.name).as_str());
        sleep(2);
        if !split {
            self.hand.hit(deck);
        } else {
            let mut hand = self.split_hand.clone().unwrap();
            hand.hit(deck);
            self.split_hand = Some(hand);
        }
    }

    pub fn split_hand(&mut self, deck: &mut Deck) {
        print_if(format!("\n{} split!", self.name).as_str());
        sleep(2);
        let mut hand = Hand {
            aces: 0,
            value: 0,
            cards: vec![self.hand.cards.remove(0), deck.cards.remove(0)],
        };
        hand.set_value();
        self.hand.cards.push(deck.cards.remove(0));
        self.hand.set_value();
        self.funds -= self.wager;
        self.wager *= 2.0;
        self.hand.print();
        hand.print();
        self.split_hand = Some(hand);
    }

    pub fn double_down(&mut self, deck: &mut Deck, split: bool) {
        print_if(format!("\n{} doubled down!", self.name).as_str());
        sleep(2);
        if !split {
            self.funds -= self.wager;
            self.wager *= 2.0;
            self.hand.hit(deck);
        } else {
            self.funds -= (self.wager / 2.0);
            self.wager *= 1.5;
            let mut hand = self.split_hand.clone().unwrap();
            hand.hit(deck);
            self.split_hand = Some(hand);
        }
    }

    pub fn turn(&mut self, deck: &mut Deck, dealer_showing: Value, count: i8) {
        print_if(format!("\n{}'s turn", self.name).as_str());
        match self.type_ {
            PlayerType::Human => self.human_turn(deck),
            PlayerType::EdgarAI => self.edgar_turn(deck, dealer_showing),
            PlayerType::DannyAI => self.edgar_turn(deck, dealer_showing),
            PlayerType::CheapoAI => self.edgar_turn(deck, dealer_showing),
            PlayerType::CheapoPAI => self.edgar_turn(deck, dealer_showing),
            PlayerType::RayAI => self.ray_turn(deck, dealer_showing, count),
        }
    }

    pub fn make_wager(&mut self, count: i8) {
        match self.type_ {
            PlayerType::Human => {
                let mut wager: String = read!("{}\n");
                let mut w: f64 = wager.parse().unwrap();
                while w > self.funds || w < 1.0 {
                    if w > self.funds {
                        print_if("You don't have that much money!");
                    } else {
                        print_if("You can't gamble your debts loser....")
                    }
                    print_if("\nHow much would you like to bet?");
                    wager = read!("{}\n");
                    w = wager.parse().unwrap();
                }
                self.wager(w);
            }
            PlayerType::EdgarAI => {
                let wager = self.funds / 100.0;
                print_if(format!("{wager}").as_str());
                self.wager(wager);
            }
            PlayerType::RayAI => {
                let wager = self.funds / 100.0;
                print_if(format!("{wager}").as_str());
                self.wager(wager);
            }
            PlayerType::DannyAI => {
                let mut wager = self.funds / 10.0;
                if count < (-4) {
                    wager *= 2.0;
                } else if count < -2 {
                    wager *= 1.5;
                } else if count > 4 {
                    wager /= 2.0;
                } else if count > 2 {
                    wager /= 1.5;
                }
                print_if(format!("{wager}").as_str());
                self.wager(wager);
            }
            PlayerType::CheapoAI => {
                if self.previous_win {
                    self.wager(0.1);
                    print_if(format!("{}", self.wager).as_str());
                } else {
                    self.wager(self.previous_wager * 2.0);
                    print_if(format!("{}", self.wager).as_str());
                }
            }
            PlayerType::CheapoPAI => {
                let factor = (self.funds / 10000.0).ceil();
                print_if(format!("\n\nfactor: {factor}\n\n").as_str());
                if self.previous_win {
                    self.wager(0.1 * factor);
                    print_if(format!("{}", self.wager).as_str());
                } else {
                    self.wager(self.previous_wager * 2.0);
                    print_if(format!("{}", self.wager).as_str());
                }
            }
        }
    }

    pub fn wager(&mut self, amount: f64) {
        let a = amount;
        let wager: f64 = Decimal::from_f64_retain(a)
            .unwrap()
            .round_dp(2)
            .to_string()
            .parse()
            .unwrap_or(0.00);
        self.wager = wager;
        self.funds -= wager;
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PlayerType {
    Human,
    EdgarAI,
    DannyAI,
    CheapoAI,
    CheapoPAI,
    RayAI,
}

#[derive(Debug, Clone)]
pub struct Stats {
    pub player_name: String,
    pub hands_played: u32,
    pub blackjacks: u32,
    pub dealer_blackjacks: u32,
    pub highest_wager: f64,
    pub max_funds: f64,
    pub winnings: f64,
    pub hands_won: u32,
    pub hands_lost: u32,
    pub hands_pushed: u32,
    pub win_percentage: f64,
}
impl Stats {
    fn default(name: String) -> Self {
        Stats {
            player_name: name,
            hands_played: 0,
            blackjacks: 0,
            dealer_blackjacks: 0,
            highest_wager: 0.0,
            max_funds: 0.0,
            winnings: 0.0,
            hands_won: 0,
            hands_lost: 0,
            hands_pushed: 0,
            win_percentage: 0.0,
        }
    }

    pub fn to_string(&self) -> String {
        format!(
            "\n\n Name: {}\nHands Played: {}\nBlackjacks: {}\nDealer Blackjacks: {}\nHighest Wager: ${}\nMax Funds: ${}\nWinnings: ${}\nHands Won: {}\nHands Lost: {}\nHands Pushed: {}\nWin Percentage: {}%",
            self.player_name,
            self.hands_played,
            self.blackjacks,
            self.dealer_blackjacks,
            self.highest_wager,
            self.max_funds,
            self.winnings,
            self.hands_won,
            self.hands_lost,
            self.hands_pushed,
            self.win_percentage,
        )
    }
    pub fn print(&self) {
        print_if("\n-----------------");
        print_if(format!("{}'s Stats", self.player_name).as_str());
        print_if("-----------------");
        print_if(format!("Hands Played: {}", self.hands_played).as_str());
        print_if(format!("Blackjacks: {}", self.blackjacks).as_str());
        print_if(format!("Dealer Blackjacks: {}", self.dealer_blackjacks).as_str());
        print_if(format!("Highest Wager: ${}", self.highest_wager).as_str());
        print_if(format!("Max Funds: ${}", self.max_funds).as_str());
        print_if(format!("Winnings: ${}", self.winnings).as_str());
        print_if(format!("Hands Won: {}", self.hands_won).as_str());
        print_if(format!("Hands Lost: {}", self.hands_lost).as_str());
        print_if(format!("Hands Pushed: {}", self.hands_pushed).as_str());
        print_if(format!("Win Percentage: {}%", self.win_percentage).as_str());
        print_if("-----------------");
    }
}
