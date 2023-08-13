use crate::player::{Player, PlayerType};

impl Player {
    pub fn new_cheapo(num: &str) -> Self {
        let mut name = "CH33P0B0T-3000".to_string();
        name.push_str("-");
        name.push_str(num);
        Self::new(name, PlayerType::CheapoAI)
    }
}
