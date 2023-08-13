use crate::player::{Player, PlayerType};
use crate::util::print_if;

impl Player {
    pub fn new_cheapo(num: i8) -> Self {
        print_if(format!("Creating Cheapo Bot {num}").as_str());
        let mut name = "CH33P0B0T-3000".to_string();
        if num > 0 {
            name.push_str("-");
            name.push_str(num.to_string().as_str());
        }
        Self::new(name, PlayerType::CheapoAI)
    }
}
