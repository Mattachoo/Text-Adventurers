use crate::character::Character;
use crate::hp::HitPointState;
use crate::io::Interface;

pub struct Target {
    name: String,
}

impl Target {
    pub fn target_character(character: &Character) -> Target {
        Target {
            name: character.name.clone(),
        }
    }
}

pub enum Action {
    Attack(Target),
}

impl Action {
    fn run<I: Interface>(&self, interface: &mut I, actor: usize, combat_frame: &mut CombatFrame) {
        match self {
            Action::Attack(target) => {
                if let Some(target_index) = combat_frame.find_target(target) {
                    {
                        let target_character = &mut combat_frame.characters[target_index];
                        target_character.hitpoints().take_damage(1);
                    }
                    let actor = &mut combat_frame.characters[actor];
                    interface.write(&format![
                        "{} attacked {} for 1 damage.",
                        actor.name, target.name
                    ]);
                } else {
                    let actor = &mut combat_frame.characters[actor];
                    // Missed
                    interface.write(&format![
                        "{} tried to attack something nonexistent.",
                        actor.name
                    ]);
                }
            }
        }
        // Do nothing
    }
}

pub struct CombatFrame<'a> {
    characters: Vec<&'a mut Character>,
}

impl<'a> CombatFrame<'a> {
    pub fn new() -> CombatFrame<'a> {
        CombatFrame {
            characters: Vec::new(),
        }
    }

    pub fn add_character(&mut self, character: &'a mut Character) {
        self.characters.push(character);
    }

    pub fn run<I: Interface>(&mut self, interface: &mut I) {
        let mut combat_over = false;
        while !combat_over {
            // Assumption: characters is not modified when actions are run.
            for i in 0..self.characters.len() {
                // TODO Need IO interface in act
                let action = self.characters[i].act(interface, &self);
                action.run(interface, i, self);
                if self.finished() {
                    combat_over = true;
                    break;
                }
            }
        }
    }

    fn finished(&mut self) -> bool {
        let mut depleted_count: usize = 0;
        for character in &mut self.characters {
            if character.hitpoints().state() == HitPointState::Depleted {
                depleted_count += 1;
            }
        }
        self.characters.len() - depleted_count <= 1
    }

    fn find_target(&self, target: &Target) -> Option<usize> {
        for i in 0..self.characters.len() {
            if self.characters[i].name == target.name {
                return Some(i);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::hp::HitPointState;
    use crate::io::TestInterface;
    use crate::stat::{StatBlock, StatKind};
    use std::collections::VecDeque;

    #[test]
    pub fn characters_take_actions_until_combat_end() {
        let mut char1 = Character::new(String::from("Char1"), StatBlock::new());
        char1
            .stats
            .mut_stat(StatKind::Constitution)
            .set_base_value(2);
        char1.hitpoints().heal_fully();
        let mut char2 = Character::new(String::from("Char2"), StatBlock::new());
        char2
            .stats
            .mut_stat(StatKind::Constitution)
            .set_base_value(3);
        char2.hitpoints().heal_fully();
        let mut frame = CombatFrame::new();
        frame.add_character(&mut char1);
        frame.add_character(&mut char2);

        let mut interface = TestInterface::new(VecDeque::new());
        frame.run(&mut interface);

        assert_eq!(
            interface.written,
            "Char1 attacked Char1 for 1 damage.
Char2 attacked Char2 for 1 damage.
Char1 attacked Char1 for 1 damage.
Char2 attacked Char2 for 1 damage.
Char1 attacked Char1 for 1 damage.
Char2 attacked Char2 for 1 damage.
Char1 attacked Char1 for 1 damage.\n"
        );
        assert_eq!(char1.hitpoints().state(), HitPointState::Depleted);
    }
}
