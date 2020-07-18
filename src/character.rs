use crate::accessible::{Accessible, Accessor};
use crate::hp::HitPoints;
use crate::stat::{StatBlock, StatKind};

pub struct Character {
    pub name: String,
    pub stats: StatBlock,
    hitpoints: HitPoints,
    accessor: Accessor<Character>,
}

impl Character {
    pub fn new(name: String, stats: StatBlock) -> Character {
        let mut accessor = Accessor::new();
        accessor.register("name", Box::new(|c: &Character| Some(c.name.clone())));
        Character {
            name,
            stats,
            hitpoints: HitPoints::new(stats.max_hp()),
            accessor,
        }
    }

    pub fn hitpoints(&mut self) -> &mut HitPoints {
        self.hitpoints.set_max(self.stats.max_hp());
        &mut self.hitpoints
    }
}

impl Accessible for Character {
    fn lookup_local(&self, property: &str) -> Option<String> {
        self.accessor.lookup(property, self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn derives_max_hitpoints_from_stats() {
        let mut stats = StatBlock::new();
        stats.mut_stat(StatKind::Constitution).set_base_value(5);
        let mut character = Character::new(String::from("TestCharacter"), stats);
        assert_eq!(character.hitpoints().max(), 25);
        character
            .stats
            .mut_stat(StatKind::Endurance)
            .set_base_value(5);
        assert_eq!(character.hitpoints().max(), 36);
    }
}
