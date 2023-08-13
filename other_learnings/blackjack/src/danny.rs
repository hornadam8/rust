use crate::player::{Player, PlayerType};

impl Player {
    pub fn new_danny(num: u8) -> Self {
        if num == 0 {
            Self::new("Danny Ocean".to_string(), PlayerType::DannyAI)
        } else {
            Self::new(format!("Danny Ocean({num})"), PlayerType::DannyAI)
        }
    }
}
