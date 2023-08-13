use crate::deck::Deck;
use crate::game::Header;
use crate::player::Player;
use crate::util::{print_if, sleep};

impl Player {
    pub fn human_turn(&mut self, deck: &mut Deck, header: Header) {
        self.print_w_header(&header);
        if self.hand.is_blackjack() {
            print_if("Blackjack! You win");
            return;
        }
        let caught = self.cheat(deck);
        if caught {
            self.hand.value = 0;
            return;
        }
        let mut valid_moves = vec!["hit", "stand"];
        if self.hand.cards[0].value == self.hand.cards[1].value && self.wager < self.funds / 2.0 {
            valid_moves.push("split");
        }
        if self.wager < self.funds / 2.0 {
            valid_moves.push("double down");
        }

        print_if("What would you like to do?");
        for i in 1..=valid_moves.len() {
            let move_ = valid_moves.get(i - 1).unwrap();
            print_if(format!("{i}) {move_}").as_str());
        }
        let selection: String = read!("{}\n");
        let mut sel: u8 = selection.parse().unwrap();

        while sel > valid_moves.len() as u8 || sel < 1 {
            print_if("invalid input");
            print_if("What would you like to do?");
            for i in 1..=valid_moves.len() {
                let move_ = valid_moves.get(i - 1).unwrap();
                print_if(format!("{i}) {move_}").as_str());
            }
            let selection: String = read!("{}\n");
            sel = selection.parse().unwrap();
        }

        let sel_move = valid_moves.remove((sel - 1) as usize);
        valid_moves = vec!["hit", "stand"];
        match sel_move {
            "hit" => self.hit(deck, false),
            "double down" => {
                self.double_down(deck, false);
                return;
            }
            "split" => self.split_hand(deck),
            _ => {
                print_if(format!("\n{} stood!", self.name).as_str());
                sleep(2);
                return;
            }
        }
        while self.hand.value > 0 {
            self.print_w_header(&header);
            print_if("What would you like to do?");
            for i in 1..=valid_moves.len() {
                let move_ = valid_moves.get(i - 1).unwrap();
                print_if(format!("{i}) {move_}").as_str());
            }
            let selection: String = read!("{}\n");
            let mut sel: u8 = selection.parse().unwrap_or(0);
            while sel > valid_moves.len() as u8 || sel < 1 {
                print_if("invalid input");
                print_if("What would you like to do?");
                for i in 1..=valid_moves.len() {
                    let move_ = valid_moves.get(i - 1).unwrap();
                    print_if(format!("{i}) {move_}").as_str());
                }
                let selection: String = read!("{}\n");
                sel = selection.parse().unwrap_or(0);
            }
            let sel_move = valid_moves.remove((sel - 1) as usize);
            valid_moves = vec!["hit", "stand"];
            match sel_move {
                "hit" => {
                    print_if(format!("\n{} hit!", self.name).as_str());
                    sleep(2);
                    self.hit(deck, false);
                }
                _ => {
                    print_if(format!("\n{} stood!", self.name).as_str());
                    sleep(2);
                    break;
                }
            }
        }
        match self.split_hand {
            Some(_) => {
                let hand = self.split_hand.clone().unwrap();
                while hand.value > 0 {
                    self.print_w_header_split(&header, &hand);
                    print_if("What would you like to do?");
                    for i in 1..=valid_moves.len() {
                        let move_ = valid_moves.get(i - 1).unwrap();
                        print_if(format!("{i}) {move_}").as_str());
                    }
                    let selection: String = read!("{}\n");
                    let mut sel: u8 = selection.parse().unwrap_or(0);
                    while sel > valid_moves.len() as u8 || sel < 1 {
                        print_if("invalid input");
                        print_if("What would you like to do?");
                        for i in 1..=valid_moves.len() {
                            let move_ = valid_moves.get(i - 1).unwrap();
                            print_if(format!("{i}) {move_}").as_str());
                        }
                        let selection: String = read!("{}\n");
                        sel = selection.parse().unwrap_or(0);
                    }
                    let sel_move = valid_moves.remove((sel - 1) as usize);
                    valid_moves = vec!["hit", "stand"];
                    match sel_move {
                        "hit" => {
                            print_if(format!("{} hit!", self.name).as_str());
                            sleep(2);
                            self.hit(deck, true);
                        }
                        _ => {
                            print_if(format!("{} stood!", self.name).as_str());
                            sleep(2);
                            self.split_hand = Some(hand);
                            return;
                        }
                    }
                }
                self.split_hand = Some(hand);
                return;
            }
            None => {
                return;
            }
        }
    }
}
