use crate::accessible::{Accessible, Accessor};
use crate::character::Character;
use crate::stat::StatBlock;

pub struct World {
    pub player: Character,
    accessor: Accessor<World>,
}

impl World {
    pub fn empty() -> World {
        World {
            player: Character::new(String::from("Player"), StatBlock::new()),
            accessor: Accessor::new(),
        }
    }
}

impl Accessible for World {
    fn lookup_local(&self, property: &str) -> Option<String> {
        self.accessor.lookup(property, self)
    }
    fn get_child(&self, child: &str) -> Option<&Accessible> {
        match child {
            "player" => Some(&self.player),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::accessible::AccessPath;

    #[test]
    fn is_accessible() {
        let world = World::empty();
        assert_eq!(
            world.lookup(AccessPath::from(String::from("player.name")).view()),
            Some(String::from("Player"))
        );
    }
}
