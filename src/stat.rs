use crate::table::{ColumnAlignment, Column, Table};

#[derive(Copy, Clone)]
pub enum StatKind {
    Strength = 0,
    Endurance,
    Constitution,
    Will,
    Intelligence,
    Perception,
    Agility,
    // Placeholder used for determining StatBlock size
    FinalStat,
}

const STAT_COUNT: usize = StatKind::FinalStat.index() + 1;
const MAX_BASE_VALUE: i64 = 30;

impl StatKind {
    const fn stat_list() -> [StatKind; STAT_COUNT] {
        [
            StatKind::Strength,
            StatKind::Endurance,
            StatKind::Constitution,
            StatKind::Will,
            StatKind::Intelligence,
            StatKind::Perception,
            StatKind::Agility,
            StatKind::FinalStat,
        ]
    }

    const fn from_usize(i: usize) -> StatKind {
        StatKind::stat_list()[i]
    }

    pub const fn index(self) -> usize {
        self as usize
    }

    pub fn ticker(self) -> &'static str {
        match self {
            StatKind::Strength => "STR",
            StatKind::Endurance => "END",
            StatKind::Constitution => "CON",
            StatKind::Will => "WIL",
            StatKind::Intelligence => "INT",
            StatKind::Perception => "PER",
            StatKind::Agility => "AGI",
            _ => self.display_name(),
        }
    }

    pub fn display_name(self) -> &'static str {
        match self {
            StatKind::Strength => "Strength",
            StatKind::Endurance => "Endurance",
            StatKind::Constitution => "Constitution",
            StatKind::Will => "Will",
            StatKind::Intelligence => "Intelligence",
            StatKind::Perception => "Perception",
            StatKind::Agility => "Agility",
            StatKind::FinalStat => "___",
        }
    }

    pub fn next_level_progress(self, previous_next_level_progress: i64) -> i64 {
        match self {
            _ => previous_next_level_progress + 100,
        }
    }

    pub fn transfer_factor(target: StatKind, source: StatKind) -> f64 {
        const MAJOR_ADJACENT: f64 = 0.2;
        match target {
            StatKind::Strength => match source {
                StatKind::Endurance => MAJOR_ADJACENT,
                StatKind::Agility => MAJOR_ADJACENT,
                _ => 0.0,
            },
            StatKind::Endurance => match source {
                StatKind::Strength => MAJOR_ADJACENT,
                StatKind::Constitution => MAJOR_ADJACENT,
                _ => 0.0,
            },
            StatKind::Constitution => match source {
                StatKind::Endurance => MAJOR_ADJACENT,
                StatKind::Will => MAJOR_ADJACENT,
                _ => 0.0,
            },
            StatKind::Will => match source {
                StatKind::Constitution => MAJOR_ADJACENT,
                StatKind::Intelligence => MAJOR_ADJACENT,
                _ => 0.0,
            },
            StatKind::Intelligence => match source {
                StatKind::Will => MAJOR_ADJACENT,
                StatKind::Perception => MAJOR_ADJACENT,
                _ => 0.0,
            },
            StatKind::Perception => match source {
                StatKind::Intelligence => MAJOR_ADJACENT,
                StatKind::Agility => MAJOR_ADJACENT,
                _ => 0.0,
            },
            StatKind::Agility => match source {
                StatKind::Perception => MAJOR_ADJACENT,
                StatKind::Strength => MAJOR_ADJACENT,
                _ => 0.0,
            },
            StatKind::FinalStat => 0.0,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Stat {
    pub kind: StatKind,
    // Ranges from 0 to 30.
    base_value: i64,
    // Starts at 0, then increases.
    // Going down would be very rare, but if it does go down,
    // it stops at zero. Decreases cannot affect base_value.
    progress: i64,
    progress_to_next_level: i64,
}

impl Stat {
    pub fn new(kind: StatKind) -> Stat {
        Stat{
            kind,
            base_value: 0,
            progress: 0,
            progress_to_next_level: 100,
        }
    }

    pub fn base_value(&self) -> i64 {
        self.base_value
    }

    // Note: not the final value. Characters may have other modifiers outside
    // of the stat block.
    pub fn value(&self, block: &StatBlock) -> i64 {
        let mut value_with_transfer = self.base_value;
        for kind in StatKind::stat_list().iter() {
            let transferred_value =
                (block.stat(*kind).base_value as f64
                * StatKind::transfer_factor(self.kind, *kind)) as i64;
            value_with_transfer += transferred_value;
        }
        value_with_transfer
    }

    pub fn set_base_value(&mut self, base_value: i64) {
        if base_value < 0 || base_value > MAX_BASE_VALUE {
            panic!("stat value out of range")
        }
        self.base_value = base_value
    }

    pub fn advance(&mut self, amount: i64) {
        self.progress += amount;
        while self.progress >= self.progress_to_next_level {
            if self.base_value == MAX_BASE_VALUE {
                return;
            }
            self.base_value += 1;
            self.progress -= self.progress_to_next_level;
            self.progress_to_next_level =
                self.kind.next_level_progress(self.progress_to_next_level);
        }
    }

    pub fn progress(&self) -> i64 {
        return self.progress;
    }

    pub fn progress_to_next_level(&self) -> i64 {
        return self.progress_to_next_level;
    }
}

#[derive(Copy, Clone)]
pub struct StatBlock {
    stats: [Stat; STAT_COUNT],
}

impl StatBlock {
    // Creates a new StatBlock with no progress.
    pub fn new() -> StatBlock {
        let mut stats =
            [Stat::new(StatKind::FinalStat); STAT_COUNT];
        for i in 0..stats.len() {
            stats[i].kind = StatKind::from_usize(i);
        }
        StatBlock{stats}
    }

    pub fn stat(&self, kind: StatKind) -> &Stat {
        &self.stats[kind.index()]
    }

    pub fn mut_stat(&mut self, kind: StatKind) -> &mut Stat {
        &mut self.stats[kind.index()]
    }

    pub fn check(&self, kind: StatKind, required: i64) -> bool {
        self.stat(kind).value(self) >= required
    }

    pub fn print_table(&self) -> String {
        let table = Table{
            columns: vec![
                Column{
                    name: "Stat",
                    extractor: Box::new(|stat: &Stat| {
                        stat.kind.display_name().to_string()
                    }),
                    alignment: ColumnAlignment::Left,
                },
                Column{
                    name: "Base",
                    extractor: Box::new(|stat: &Stat| {
                        stat.base_value().to_string()
                    }),
                    alignment: ColumnAlignment::Right,
                },
                Column{
                    name: "Modified",
                    extractor: Box::new(|stat: &Stat| {
                        stat.value(&self).to_string()
                    }),
                    alignment: ColumnAlignment::Right,
                },
                Column{
                    name: "Progress",
                    extractor: Box::new(|stat: &Stat| {
                        stat.progress().to_string()
                    }),
                    alignment: ColumnAlignment::Right,
                },
                Column{
                    name: "Next Level",
                    extractor: Box::new(|stat: &Stat| {
                        stat.progress_to_next_level().to_string()
                    }),
                    alignment: ColumnAlignment::Right,
                },
            ]
        };
        table.render(self.stats.iter().filter(|stat| stat.value(self) != 0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn prints_tables() {
        let mut block = StatBlock::new();
        block.mut_stat(StatKind::Strength).set_base_value(5);
        assert_eq!(block.print_table(), String::from(
"| Stat      | Base | Modified | Progress | Next Level |
| Strength  |    5 |        5 |        0 |        100 |
| Endurance |    0 |        1 |        0 |        100 |
| Agility   |    0 |        1 |        0 |        100 |
"
                   ),
                   "\nGot table:\n{}",
                   block.print_table());
    }
}
