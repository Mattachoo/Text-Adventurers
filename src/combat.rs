use crate::character::Character;
use crate::choice::Choice;
use crate::hp::HitPointState;
use crate::io::Interface;
use crate::stat::StatKind;
use crate::table::{Column, ColumnAlignment, Table};

use std::fmt;

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

impl fmt::Display for Target {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

pub enum Action {
    Attack(Target),
    Idle,
}

impl Action {
    fn run<I: Interface>(&self, interface: &mut I, actor: usize, combat_frame: &mut CombatFrame) {
        match self {
            Action::Attack(target) => {
                if let Some(target_index) = combat_frame.find_target(target) {
                    let mut strength = 0;
                    {
                        let actor = &mut combat_frame.characters[actor];
                        strength = actor.stats.stat(StatKind::Strength).value(&actor.stats);
                    }
                    let damage = 1 + strength;
                    {
                        let target_character = &mut combat_frame.characters[target_index];
                        target_character.hitpoints().take_damage(damage);
                    }
                    let actor = &mut combat_frame.characters[actor];
                    interface.write(&format![
                        "{} attacked {} for {} damage.",
                        actor.name, target.name, damage
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
            Action::Idle => {
                let actor = &mut combat_frame.characters[actor];
                interface.write(&format!["{} dawdles.", actor.name]);
            }
        }
    }
}

impl Choice for Action {
    fn describe(&self) -> String {
        match self {
            Action::Attack(target) => format!["Attack {}", target],
            Action::Idle => String::from("Do nothing"),
        }
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
                self.summarize_status(interface);
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

    pub fn find_target_character(&self, target: &Target) -> Option<&Character> {
        for i in 0..self.characters.len() {
            if self.characters[i].name == target.name {
                return Some(self.characters[i]);
            }
        }
        None
    }

    fn summarize_status<I: Interface>(&self, interface: &mut I) {
        let table: Table<Character> = Table {
            columns: vec![
                Column {
                    name: "Name",
                    extractor: Box::new(|character: &Character| character.name.clone()),
                    alignment: ColumnAlignment::Left,
                },
                Column {
                    name: "HP",
                    extractor: Box::new(|character: &Character| {
                        format!("{}", character.hitpoints_snapshot().current())
                    }),
                    alignment: ColumnAlignment::Right,
                },
                Column {
                    name: "Max HP",
                    extractor: Box::new(|character: &Character| {
                        format!("{}", character.hitpoints_snapshot().max())
                    }),
                    alignment: ColumnAlignment::Right,
                },
            ],
        };
        interface.write(
            &table.render(
                self.characters
                    .iter()
                    .map(|character: &&mut Character| &(**character)),
            ),
        );
    }

    pub fn list_targets(&self) -> Vec<Target> {
        self.characters
            .iter()
            .map(|character| Target::target_character(character))
            .collect()
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
        char1.stats.mut_stat(StatKind::Strength).set_base_value(2);
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
        // Newline for nicer table layout in test.
        interface.write("");
        frame.run(&mut interface);

        assert_eq!(
            interface.written,
            "
| Name  | HP | Max HP |
| Char1 |  4 |      4 |
| Char2 |  9 |      9 |

Char1 attacked Char2 for 3 damage.
| Name  | HP | Max HP |
| Char1 |  4 |      4 |
| Char2 |  6 |      9 |

Char2 attacked Char1 for 1 damage.
| Name  | HP | Max HP |
| Char1 |  3 |      4 |
| Char2 |  6 |      9 |

Char1 attacked Char2 for 3 damage.
| Name  | HP | Max HP |
| Char1 |  3 |      4 |
| Char2 |  3 |      9 |

Char2 attacked Char1 for 1 damage.
| Name  | HP | Max HP |
| Char1 |  2 |      4 |
| Char2 |  3 |      9 |

Char1 attacked Char2 for 3 damage.\n"
        );
        assert_eq!(char2.hitpoints().state(), HitPointState::Depleted);
    }

    #[test]
    pub fn players_choose_actions() {
        let mut char1 = Character::new_player(String::from("Player"), StatBlock::new());
        let mut char2 = Character::new(String::from("Char2"), StatBlock::new());

        let mut frame = CombatFrame::new();
        frame.add_character(&mut char1);
        frame.add_character(&mut char2);

        let mut interface = TestInterface::new(VecDeque::from(vec![1]));
        // Newline for nicer table layout in test.
        interface.write("");
        frame.run(&mut interface);

        assert_eq!(
            interface.written,
            "
| Name   | HP | Max HP |
| Player |  1 |      1 |
| Char2  |  1 |      1 |

Player attacked Char2 for 1 damage.\n"
        );
    }
}
