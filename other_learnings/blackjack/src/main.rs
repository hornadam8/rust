mod card;
mod cheapo;
mod cheapo_p;
mod danny;
mod dealer;
mod deck;
mod edgar;
mod game;
mod hand;
mod human;
mod player;
mod ray;
mod util;

use crate::util::{print_if, sleep};
use game::Game;
use std::env;

#[macro_use]
extern crate text_io;

fn main() {
    let mut cont: bool = true;
    env::set_var("FAST", "false");
    print_if("Would you like to play in always_continue mode? (y/n)");
    let input: String = read!("{}\n");
    let mut always = false;
    if input == "y" {
        always = true;
    }

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
    let mut game = Game::new();
    let mut hand_count: u32 = 0;
    while (cont || always) && game.players.len() > 0 {
        hand_count += 1;
        print_if(
            format!(
                "Starting hand #{}\nPlayers left: {}",
                hand_count,
                game.players.len()
            )
            .as_str(),
        );
        sleep(2);
        game.play();
        if !always {
            print_if("Continue? (y/n)");
            let input: String = read!("{}\n");
            if input == "n" {
                cont = false;
            }
        }
    }
    game.update_stats();
    print_if("\n\nGame over!\n\n");
    print_if("\n\nStats:\n");
    for (_, stats) in game.stats.iter() {
        stats.print();
    }
}
