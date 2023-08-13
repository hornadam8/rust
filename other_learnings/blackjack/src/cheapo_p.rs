use crate::player::{Player, PlayerType};
use crate::util::print_if;

impl Player {
    pub fn new_cheapo_p(num: i8) -> Self {
        print_if(format!("Creating Cheapo Bot-P {num}").as_str());
        let mut name = format!("CH33P0B0T-3000-P");
        if num > 0 {
            name.push_str("-");
            name.push_str(num.to_string().as_str());
        }
        Self::new(name, PlayerType::CheapoPAI)
    }
}
