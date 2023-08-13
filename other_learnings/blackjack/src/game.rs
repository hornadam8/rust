////////////////////////////////////////////////////////////////////////////////////////////////////
//                                            Game
////////////////////////////////////////////////////////////////////////////////////////////////////

use crate::dealer::Dealer;
use crate::deck::Deck;
use crate::hand::Hand;
use crate::player::{Player, PlayerType, Stats};
use crate::util::{clear_terminal, print_if, sleep};
use rust_decimal::Decimal;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};

pub struct Game {
    pub players: Vec<Player>,
    pub dealer: Dealer,
    pub deck: Deck,
    pub deck_count: u8,
    pub true_count: i8,
    pub running_count: i8,
    pub hand_num: i32,
    pub stats: HashMap<String, Stats>,
}
impl Game {
    pub fn new() -> Self {
        let dealer = Dealer::new();
        let mut players = vec![];
        println!("\n\n\n");
        println!("#########################################################");
        println!("# Welcome to Rusty Blackjack you degenerate bastards <3 #");
        println!("#########################################################");
        println!("\n\n");
        sleep(3);
        println!("       Rules        \n-------------------");
        println!("2-6 decks");
        println!("Dealer hits on soft 17");
        println!("Blackjack pays 3:2");
        println!("A hand can only be split once");
        println!("\n\n\n");
        sleep(3);

        println!("\n\nHow many decks would you like to play with? 2, 4, 6, or 8?");
        let num_decks: String = read!("{}\n");
        let mut nd: u8 = num_decks.parse().unwrap();
        while nd < 2 || nd > 8 || nd % 2 != 0 {
            println!("invalid input!");
            println!("How many decks would you like to play with? 2, 4, 6, or 8?");
            let num_decks: String = read!("{}\n");
            nd = num_decks.parse().unwrap();
        }

        println!("Would you like to add AI players to your game? (y/n)");
        let input: String = read!("{}\n");
        if input == "y" {
            let edgar = Player::new_edgar(0);
            players.push(edgar);
            let ray = Player::new_ray(0);
            players.push(ray);
            let danny = Player::new_danny(0);
            players.push(danny);
            let cheapo = Player::new_cheapo(0);
            players.push(cheapo);
            let cheapo_p = Player::new_cheapo_p(0);
            players.push(cheapo_p);
        }

        println!("\n\nHow many human players are playing? (0-8):");
        let num_players: String = read!("{}\n");
        let mut np: i32 = num_players.parse().unwrap();

        while np < 0 || np > 8 {
            println!("invalid input!");
            println!("How many players are playing? (1-8):");
            let num_players: String = read!("{}\n");
            np = num_players.parse().unwrap();
        }

        for i in 1..=np {
            println!("\n\nEnter a name for Player {i}:");
            let name: String = read!("{}\n");
            let player = Player::new(name, PlayerType::Human);
            players.push(player)
        }
        println!("\n\n\n");
        Self {
            players,
            dealer,
            deck: Deck::new(nd),
            deck_count: nd,
            running_count: 0,
            true_count: 0,
            hand_num: 0,
            stats: HashMap::new(),
        }
    }

    fn new_deck(&mut self) {
        let mut deck = Deck::new(self.deck_count);
        deck.shuffle();
        self.deck = deck;
    }

    fn get_wagers(&mut self) {
        let count = self.true_count;
        for player in self.players.iter_mut() {
            let name = &player.name;
            print_if(format!("\n\nIt's {name}'s turn to bet!").as_str());
            player.print();
            sleep(1);
            print_if("How much would you like to bet?");
            player.make_wager(count);
            if player.wager > player.stats.highest_wager {
                player.stats.highest_wager = player.wager;
            }
        }
    }

    fn evaluate_players(&mut self) {
        let dv = self.dealer.hand.value;
        for player in self.players.iter_mut() {
            match &player.split_hand {
                Some(split) => {
                    player.previous_wager = player.wager / 2.0;
                    player.stats.hands_played += 1;
                    if player.hand.is_blackjack() {
                        player.stats.blackjacks += 1;
                        player.stats.hands_won += 1;
                        player.stats.winnings += (player.wager / 2.0) * 2.5;
                        player.funds += (player.wager / 2.0) * 1.5;
                        player.wager /= 2.0;
                    }
                    if player.hand.value == 0 || player.hand.value < dv {
                        player.stats.hands_lost += 1;
                        player.wager /= 2.0;
                    } else if player.hand.value == dv {
                        player.stats.hands_pushed += 1;
                        player.wager /= 2.0;
                        player.funds += player.wager;
                    } else {
                        player.stats.hands_won += 1;
                        player.stats.winnings += player.wager;
                        player.funds += player.wager;
                        player.wager /= 2.0;
                    }

                    if split.value == 0 || split.value < dv {
                        player.stats.hands_lost += 1;
                        player.wager = 0.0;
                    } else if split.is_blackjack() {
                        player.stats.blackjacks += 1;
                        player.stats.hands_won += 1;
                        player.stats.winnings += player.wager * 2.5;
                        player.funds += player.wager * 2.5;
                        player.wager = 0.0;
                    } else if split.value == dv {
                        player.stats.hands_pushed += 1;
                        player.funds += player.wager;
                        player.wager = 0.0;
                    } else {
                        player.stats.hands_won += 1;
                        player.stats.winnings += player.wager * 2.0;
                        player.funds += player.wager * 2.0;
                        player.wager = 0.0;
                    }
                }
                None => {
                    player.previous_wager = player.wager;
                    if player.hand.is_blackjack() {
                        player.stats.blackjacks += 1;
                        player.stats.hands_won += 1;
                        player.stats.winnings += player.wager * 2.5;
                        player.funds += player.wager * 2.5;
                        player.previous_win = true;
                        player.wager = 0.0;
                    }
                    if player.hand.value == 0 || player.hand.value < dv {
                        player.stats.hands_lost += 1;
                        player.previous_win = false;
                        player.wager = 0.0;
                    } else if player.hand.value == dv {
                        player.stats.hands_pushed += 1;
                        player.previous_win = true; // so wager stays the same after push
                        player.funds += player.wager;
                        player.wager = 0.0;
                    } else {
                        player.stats.hands_won += 1;
                        player.previous_win = true;
                        player.stats.winnings += player.wager * 2.0;
                        player.funds += player.wager * 2.0;
                        player.wager = 0.0;
                    }
                }
            }
            player.stats.win_percentage = player.stats.hands_won as f64
                / (player.stats.hands_won + player.stats.hands_lost) as f64
                * 100.0;
            player.funds = Decimal::from_f64_retain(player.funds)
                .unwrap()
                .round_dp(2)
                .to_string()
                .parse()
                .unwrap_or(0.00);
            player.stats.current_funds = player.funds;
            if player.funds > player.stats.max_funds {
                player.stats.max_funds = player.funds;
            }
            player.print();
            player.hand.print();
            sleep(3);
            player.hand = Hand::default();
            player.split_hand = None;
        }
    }
    pub fn update_stats(&mut self) {
        let mut stats_str = String::new();
        for player in self.players.iter() {
            let mut stats = player.stats.clone();
            stats.winnings = Decimal::from_f64_retain(stats.winnings)
                .unwrap()
                .round_dp(2)
                .to_string()
                .parse()
                .unwrap_or(0.00);

            self.stats.insert(player.name.clone(), stats);
        }
        for stats in self.stats.values() {
            stats_str.push_str(stats.to_string().as_str());
        }
        let f = File::create("stats/log.txt");
        if let Ok(f) = f {
            let mut f = BufWriter::new(f);
            match f.write_all(stats_str.as_bytes()) {
                Ok(_) => {
                    print_if("Successfully updated stats\n");
                }
                Err(_) => print_if("Failed to update stats\nMake sure the stats folder exists\n"),
            }
        };
    }
    fn remove_broke_players(&mut self) {
        let mut i = 0;
        while i < self.players.len() {
            let player = self.players.get(i).unwrap();
            if player.funds < 1.0 {
                let mut stats = player.stats.clone();
                stats.winnings = Decimal::from_f64_retain(stats.winnings)
                    .unwrap()
                    .round_dp(2)
                    .to_string()
                    .parse()
                    .unwrap_or(0.00);
                self.stats.insert(player.name.clone(), stats);
                let name = &player.name;
                print_if(format!("\n\n\n{name} has been kicked out for being BROKE\n\n").as_str());
                player.stats.print();
                sleep(1);
                self.players.remove(i);
            }
            i += 1;
        }
    }

    pub fn play(&mut self) {
        self.hand_num += 1;
        self.update_stats();
        if self.deck.cards.len() < (self.deck_count as usize * 52) * 8 / 10 {
            print_if(
                "\n* Dealer returns cards to the back of the shoe, the shuffling machine resets the deck *\n\
                ");
            self.running_count = 0;
            self.true_count = 0;
            self.new_deck();
        }
        self.get_wagers();
        print_if("\n\nThe dealer deals the cards\n\n");
        self.dealer.hand = self.deck.deal();
        sleep(2);
        self.dealer.print_first();
        sleep(2);
        if self.dealer.hand.is_blackjack() {
            print_if("Dealer blackjack!\n\n");
            for p in self.players.iter_mut() {
                p.wager = 0.0;
                p.stats.hands_played += 1;
                p.stats.dealer_blackjacks += 1;
                p.stats.hands_lost += 1;
            }
            return;
        }
        for player in self.players.iter_mut() {
            let name = &player.name;
            print_if(format!("\n{name} is dealt two cards", name = name).as_str());
            sleep(3);
            player.hand = self.deck.deal();
            player.hand.print();
        }
        sleep(3);
        let mut current_count = 0;
        current_count += self.true_count;
        let mut players = self.players.clone();
        for player in self.players.iter_mut() {
            players = players
                .into_iter()
                .filter(|p| p.name != player.name)
                .collect();
            player.stats.hands_played += 1;
            let header = Game::get_header(&players, self.dealer.hand.dealer_card_string());
            player.turn(
                &mut self.deck,
                self.dealer.first_card_val(),
                current_count,
                header,
            );
            print_if("\n\n");
            current_count += player.hand.get_count();
            if let Some(hand) = &player.split_hand {
                current_count += hand.get_count();
            }
            players.push(player.clone());
        }

        self.dealer.turn(&mut self.deck);
        self.set_counts();
        let headers =
            Game::get_header(self.players.as_ref(), self.dealer.hand.dealer_card_string());
        headers.print_force(self.hand_num);
        self.evaluate_players();
        print_if("Dealer had: ");
        self.dealer.hand.print();
        self.remove_broke_players();
    }

    pub fn get_header(players: &Vec<Player>, dealer_cards: String) -> Header {
        let mut message_body_1 = String::new();
        let mut messge_body_2 = String::new();
        let mut message_border_head = String::new();
        let mut message_border_foot = String::new();
        for player in players.iter() {
            let (body_1, body_2, border_head, border_foot) = player.inline_print();
            message_body_1.push_str((body_1 + " ").as_str());
            messge_body_2.push_str((body_2 + " ").as_str());
            message_border_head.push_str((border_head + " ").as_str());
            message_border_foot.push_str((border_foot + " ").as_str());
        }
        Header::new(
            message_border_head,
            message_body_1,
            messge_body_2,
            message_border_foot,
            dealer_cards,
        )
    }

    fn set_counts(&mut self) {
        let mut count: i8 = 0;
        for player in self.players.iter() {
            count += player.hand.get_count();
            if let Some(hand) = &player.split_hand {
                count += hand.get_count();
            }
        }
        count += self.dealer.hand.get_count();
        self.running_count += count;
        self.true_count =
            self.running_count / ((self.deck.cards.len() as i32 * 80 / 100) / 52) as i8;
    }
}

pub struct Header {
    pub top_border: String,
    pub body_1: String,
    pub body_2: String,
    pub bottom_border: String,
    pub dealer_cards: String,
}
impl Header {
    pub fn new(
        top_border: String,
        body_1: String,
        body_2: String,
        bottom_border: String,
        dealer_cards: String,
    ) -> Header {
        Header {
            top_border,
            body_1,
            body_2,
            bottom_border,
            dealer_cards,
        }
    }
    pub fn print(&self) {
        print_if("\n\n\n\n");
        print_if(self.top_border.as_str());
        print_if(self.body_1.as_str());
        print_if(self.body_2.as_str());
        print_if(self.bottom_border.as_str());
        print_if("\n\n");
        print_if(self.dealer_cards.as_str());
        print_if("\n\n\n\n\n");
    }

    pub fn print_force(&self, hands_played: i32) {
        // this header should only print in bot mode (when nothing else does)
        if env::var("PRINT").unwrap() == "false" && hands_played % 100 == 0 {
            clear_terminal();
            println!("{}", self.top_border.as_str());
            println!("{}", self.body_1.as_str());
            println!("{}", self.body_2.as_str());
            println!("{}", self.bottom_border.as_str());
        }
    }
}
