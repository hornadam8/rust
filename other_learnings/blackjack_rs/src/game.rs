use crate::dealer::Dealer;
use crate::deck::Deck;
use crate::hand::Hand;
use crate::player::{Player, PlayerType, Stats};
use crate::util::{print_if, sleep};
use rust_decimal::Decimal;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use text_io::read;

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
        print_if("Would you like to play in fast mode(no sleep)? (y/n)");
        let input: String = read!("{}\n");
        if input == "y" {
            env::set_var("FAST", "true");
        } else {
            env::set_var("FAST", "false");
        }

        print_if("Would you like to play in silent mode(no logging)? (y/n)\nThis is recommended for testing bots only");
        let input: String = read!("{}\n");
        if input == "y" {
            env::set_var("PRINT", "false")
        } else {
            env::set_var("PRINT", "true");
        }
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
            let edgar = Player::new_edgar(1);
            players.push(edgar);
            let ray = Player::new_ray(1);
            players.push(ray);
            let edgar = Player::new_edgar(2);
            players.push(edgar);
            let ray = Player::new_ray(2);
            players.push(ray);
            let edgar = Player::new_edgar(3);
            players.push(edgar);
            let ray = Player::new_ray(3);
            players.push(ray);
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
        let f = File::create("stats/log.txt").expect("Unable to create file");
        let mut f = BufWriter::new(f);
        f.write_all(stats_str.as_bytes())
            .expect("Unable to write data");
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
                print_if("\n\n\n{name} has been kicked out for being BROKE\n\n");
                player.stats.print();
                sleep(1);
                self.players.remove(i);
            }
            i += 1;
        }
    }

    pub fn play(&mut self) {
        self.hand_num += 1;
        if self.hand_num % 1000 == 0 {
            self.update_stats();
        }
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
        for player in self.players.iter_mut() {
            player.stats.hands_played += 1;
            player.print_w_wager();
            player.hand.print();
            sleep(2);
            self.dealer.print_first();
            sleep(2);
            player.turn(&mut self.deck, self.dealer.first_card_val(), current_count);
            print_if("\n\n");
            current_count += player.hand.get_count();
            if let Some(hand) = &player.split_hand {
                current_count += hand.get_count();
            }
        }

        self.dealer.turn(&mut self.deck);
        self.set_counts();
        self.evaluate_players();
        print_if("Dealer had: ");
        self.dealer.hand.print();
        self.remove_broke_players();
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
