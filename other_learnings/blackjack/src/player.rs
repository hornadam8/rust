use crate::card::Value;
use crate::deck::Deck;
use crate::game::Header;
use crate::hand::Hand;
use crate::util::{print_if, sleep};
use rand::Rng;
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

    pub fn print_w_header(&self, header: &Header) {
        header.print();
        self.print_w_wager();
        print_if(self.hand.card_string().as_str());
        print_if("\n");
    }

    pub fn print_w_header_split(&self, header: &Header, hand: &Hand) {
        header.print();
        self.print_w_wager();
        print_if(hand.card_string().as_str());
        print_if("\n");
    }

    pub fn inline_print(&self) -> (String, String, String, String) {
        let mut message_body_1 = format!("{} | ${}", self.name, self.wager);
        let mut message_body_2 = format!("${} | {}", self.funds, self.hand.inline_print());
        let mut line_len = 0;
        let body_1_len = message_body_1.chars().count();
        let body_2_len = message_body_2.chars().count();
        if body_1_len > body_2_len {
            line_len = body_1_len;
            let mut diff = body_1_len - body_2_len;
            if diff % 2 == 1 {
                message_body_2.push_str(" ");
                diff -= 1;
            }
            for _ in 0..diff / 2 {
                message_body_2.insert_str(0, " ");
                message_body_2.push_str(" ");
            }
        } else if body_2_len > body_1_len {
            line_len = body_2_len;
            let mut diff = body_2_len - body_1_len;
            if diff % 2 == 1 {
                message_body_1.push_str(" ");
                diff -= 1;
            }
            for _ in 0..diff / 2 {
                message_body_1.insert_str(0, " ");
                message_body_1.push_str(" ");
            }
        } else {
            line_len = body_1_len;
        }
        message_body_1.insert_str(0, " ");
        message_body_1.push_str(" ");
        message_body_2.insert_str(0, " ");
        message_body_2.push_str(" ");
        message_body_1.insert_str(0, "|");
        message_body_1.push_str("|");
        message_body_2.insert_str(0, "|");
        message_body_2.push_str("|");
        line_len += 4;
        let mut border_head = String::new();
        let mut border_foot = String::new();
        border_head.push('┌');
        border_foot.push('└');
        for _ in 0..line_len - 2 {
            border_head.push_str("─");
            border_foot.push_str("─");
        }
        border_head.push('┐');
        border_foot.push('┘');
        (message_body_1, message_body_2, border_head, border_foot)
    }

    pub fn cheat(&mut self, deck: &mut Deck) -> bool {
        let chance = rand::thread_rng().gen_range(1..=30);
        let mut caught: bool = false;
        if chance == 1 {
            caught = self.steal_chips();
        } else if chance == 2 {
            caught = self.peek(deck);
        } else {
        }
        caught
    }

    pub fn peek(&mut self, deck: &mut Deck) -> bool {
        let chance = rand::thread_rng().gen_range(45..=55);
        print_if("\n------------------------------------------------------------------");
        print_if("The dealer is distracted momentarily");
        print_if(
            format!("\n\nWould you like to peek at the next few cards? (y/n)\n\n    {chance}% chance of success\n\n ").as_str()
        );
        let input: String = read!("{}\n");
        if input == "y" {
            let peek = rand::thread_rng().gen_range(1..=100);
            if peek <= chance {
                let mut hand = Hand::default();
                let num_cards = rand::thread_rng().gen_range(2..=4);
                print_if(
                    format!("\nYou successfully peeked at the next {num_cards} cards!").as_str(),
                );
                for i in 0..num_cards {
                    hand.cards.push(deck.cards[i].clone());
                }
                print_if(format!("{}", hand.card_string()).as_str());
                sleep(3);
            } else {
                print_if("\nYou were caught and thrown out of the casino");
                self.funds = 0.0;
                return true;
            }
        }
        return false;
    }

    pub fn steal_chips(&mut self) -> bool {
        let chance = rand::thread_rng().gen_range(15..=35);
        print_if("\n------------------------------------------------------------------");
        print_if("The dealer is distracted and turns to address a ruckus at the bar.");
        print_if(
            format!("\n\nWould you like to steal some chips from the dealer?\n\n    {chance}% chance of success (y/n)\n\n ").as_str()
        );
        let input: String = read!("{}\n");
        if input == "y" {
            let steal = rand::thread_rng().gen_range(1..=100);
            if steal <= chance {
                let chips = rand::thread_rng().gen_range(500..=10000);
                print_if(format!("\nYou successfully stole ${} from the dealer!", chips).as_str());
                self.funds += chips as f64;
            } else {
                print_if("\nYou were caught and thrown out of the casino!");
                self.funds = 0.0;
                return true;
            }
        }
        false
    }

    pub fn print_w_wager(&self) {
        let message = format!("{} | ${} | Wager: ${}", self.name, self.funds, self.wager);
        let line_len = message.chars().count();
        let mut border = String::new();
        for _ in 0..line_len {
            border.push_str("─");
        }
        print_if(border.as_str());
        print_if(format!("{} | ${} | Wager: ${}", self.name, self.funds, self.wager).as_str());
        print_if(border.as_str());
    }

    pub fn hit(&mut self, deck: &mut Deck, split: bool) {
        print_if(format!("\n{} hit!", self.name).as_str());
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
        if !split {
            self.funds -= self.wager;
            self.wager *= 2.0;
            self.hand.hit(deck);
        } else {
            self.funds -= self.wager / 2.0;
            self.wager *= 1.5;
            let mut hand = self.split_hand.clone().unwrap();
            hand.hit(deck);
            self.split_hand = Some(hand);
        }
    }

    pub fn turn(&mut self, deck: &mut Deck, dealer_showing: Value, count: i8, header: Header) {
        match self.type_ {
            PlayerType::Human => self.human_turn(deck, header),
            PlayerType::EdgarAI => self.edgar_turn(deck, dealer_showing, header),
            PlayerType::DannyAI => self.edgar_turn(deck, dealer_showing, header),
            PlayerType::CheapoAI => self.edgar_turn(deck, dealer_showing, header),
            PlayerType::CheapoPAI => self.edgar_turn(deck, dealer_showing, header),
            PlayerType::RayAI => self.ray_turn(deck, dealer_showing, count, header),
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
    pub current_funds: f64,
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
            current_funds: 0.0,
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
            "\n\n Name: {}\nCurrent Funds: {}\nHands Played: {}\nBlackjacks: {}\nDealer Blackjacks: {}\nHighest Wager: ${}\nMax Funds: ${}\nWinnings: ${}\nHands Won: {}\nHands Lost: {}\nHands Pushed: {}\nWin Percentage: {}%",
            self.player_name,
            self.current_funds,
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
        print_if("\n--------------------------------");
        print_if(format!("       {}'s Stats      ", self.player_name).as_str());
        print_if("-----------------------------------");
        print_if(format!("Current Funds: ${}", self.current_funds).as_str());
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
