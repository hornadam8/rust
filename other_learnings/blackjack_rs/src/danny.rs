use crate::player::{Player, PlayerType};

impl Player {
    pub fn new_danny() -> Self {
        Self::new("Danny Ocean".to_string(), PlayerType::DannyAI)
    }
}
