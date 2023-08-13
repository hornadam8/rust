use crate::player::{Player, PlayerType};

impl Player {
    pub fn new_cheapo_p(num: &str) -> Self {
        let mut name = "CH33P0B0T-3000-P".to_string();
        name.push_str("-");
        name.push_str(num);
        Self::new(name, PlayerType::CheapoPAI)
    }
}
