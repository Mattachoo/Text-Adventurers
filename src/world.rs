use crate::character::Character;
use crate::stat::StatBlock;

pub struct World {
    pub player: Character,
}

impl World {
    pub fn empty() -> World {
        World {
            player: Character {
                stats: StatBlock::new(),
            },
        }
    }
}
