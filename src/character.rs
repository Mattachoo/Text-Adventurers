use crate::accessible::{Accessible, Accessor};
use crate::combat::{Action, CombatFrame, Target};
use crate::hp::HitPoints;
use crate::io::Interface;
use crate::stat::StatBlock;

enum Controller {
    Player,
    Ai,
}

pub struct Character {
    pub name: String,
    pub stats: StatBlock,
    hitpoints: HitPoints,
    controller: Controller,
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
            controller: Controller::Ai,
            accessor,
        }
    }

    pub fn new_player(name: String, stats: StatBlock) -> Character {
        let mut character = Character::new(name, stats);
        character.controller = Controller::Player;
        character
    }

    pub fn hitpoints(&mut self) -> &mut HitPoints {
        self.hitpoints.set_max(self.stats.max_hp());
        &mut self.hitpoints
    }

    pub fn act<I: Interface>(&self, interface: &mut I, combat_frame: &CombatFrame) -> Action {
        match self.controller {
            Controller::Player => self.ask_player_for_action(interface, combat_frame),
            Controller::Ai => self.select_action_for_npc(combat_frame),
        }
    }

    fn ask_player_for_action<I: Interface>(
        &self,
        interface: &mut I,
        combat_frame: &CombatFrame,
    ) -> Action {
        interface.choose(
            combat_frame
                .list_targets()
                .into_iter()
                .map(|target| Action::Attack(target))
                .collect(),
        )
    }

    fn select_action_for_npc(&self, combat_frame: &CombatFrame) -> Action {
        Action::Attack(Target::target_character(self))
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
    use crate::stat::StatKind;

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
