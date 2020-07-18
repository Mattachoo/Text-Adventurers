use crate::table::{Column, ColumnAlignment, Table};

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

    pub fn progress_for_level(self, level: i64) -> i64 {
        match self {
            StatKind::Strength => major_ring_advancement(level),
            StatKind::Endurance => major_ring_advancement(level),
            StatKind::Constitution => major_ring_advancement(level),
            StatKind::Will => major_ring_advancement(level),
            StatKind::Intelligence => major_ring_advancement(level),
            StatKind::Perception => major_ring_advancement(level),
            StatKind::Agility => major_ring_advancement(level),
            _ => minor_advancement(level),
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

const PROGRESS_UNITS_PER_LEVEL: i64 = 100;

// The advancement curves for progression are exponential, but they are slowed
// down with a constant strech.
// Early levels will advance with sublinear increases in XP requirements, so
// if characters can find tasks of suitable difficult they can level *faster*
// as they increase in skill.
// Later, the curve catches up and eventually provides a soft cap on level that
// varies based on the strech factor.
fn advancement_curve(point: i64, strech: f64) -> f64 {
    std::f64::consts::E.powf((point as f64) * strech)
}

fn major_ring_advancement(level: i64) -> i64 {
    (PROGRESS_UNITS_PER_LEVEL as f64 * advancement_curve(level - 1, 0.13)).ceil() as i64
}

fn minor_advancement(level: i64) -> i64 {
    // A slower strech allows stats on the minor advancement curve to reach higher
    // levels before the curve runs away (a higher soft cap).
    (PROGRESS_UNITS_PER_LEVEL as f64 * advancement_curve(level - 1, 0.08)).ceil() as i64
}

#[derive(Copy, Clone)]
pub struct ProgressCheck {
    pub required: i64,
    pub base_progress: i64,
}

fn check_progress_gain(check: ProgressCheck, skill_value: i64) -> i64 {
    // Higher values lead to faster dropoff after the ideal level.
    let dropoff_factor = 0.05;
    // Relative to the required stat, the ideal level for progression.
    let ideal_level_location = -2;
    let denominator = dropoff_factor * (skill_value - ideal_level_location - check.required) as f64;
    // Minimum progress, scaled by level (not yet units per level).
    let minimum_progress = (1.0 / denominator).max(0.0);
    // We've dealt with one asymptote, but with have a second to trim.
    let unscaled_progress = minimum_progress.min(check.required as f64);
    // We still need to scale the progress from level to progress unit, and
    // since base_progress is specified in units progress, we can simply use
    // it.
    let scaled_progress = check.base_progress as f64 * unscaled_progress;
    scaled_progress.floor() as i64
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
        Stat {
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
            let transferred_value = (block.stat(*kind).base_value as f64
                * StatKind::transfer_factor(self.kind, *kind))
                as i64;
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
            self.progress_to_next_level = self.kind.progress_for_level(self.base_value);
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
        let mut stats = [Stat::new(StatKind::FinalStat); STAT_COUNT];
        for i in 0..stats.len() {
            stats[i].kind = StatKind::from_usize(i);
        }
        StatBlock { stats }
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

    pub fn check_with_progression(&mut self, kind: StatKind, check: ProgressCheck) -> bool {
        let success = self.check(kind, check.required);
        // Using the modified value means that having transfer stats makes
        // catching up in other stats easier because characters can perform
        // more difficult checks to get more progress.
        // It also makes transfers contribute towards progress caps - progress
        // diminishes if transferred stats make the check to easy.
        let progress = check_progress_gain(check, self.stat(kind).value(self));
        self.mut_stat(kind).advance(progress);
        success
    }

    pub fn max_hp(&self) -> i64 {
        let constitution_value: i64 = self.stat(StatKind::Constitution).value(self);
        constitution_value * constitution_value
    }

    pub fn print_table(&self) -> String {
        let table = Table {
            columns: vec![
                Column {
                    name: "Stat",
                    extractor: Box::new(|stat: &Stat| stat.kind.display_name().to_string()),
                    alignment: ColumnAlignment::Left,
                },
                Column {
                    name: "Base",
                    extractor: Box::new(|stat: &Stat| stat.base_value().to_string()),
                    alignment: ColumnAlignment::Right,
                },
                Column {
                    name: "Modified",
                    extractor: Box::new(|stat: &Stat| stat.value(&self).to_string()),
                    alignment: ColumnAlignment::Right,
                },
                Column {
                    name: "Progress",
                    extractor: Box::new(|stat: &Stat| stat.progress().to_string()),
                    alignment: ColumnAlignment::Right,
                },
                Column {
                    name: "Next Level",
                    extractor: Box::new(|stat: &Stat| stat.progress_to_next_level().to_string()),
                    alignment: ColumnAlignment::Right,
                },
            ],
        };
        table.render(self.stats.iter().filter(|stat| stat.value(self) != 0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn defaults_each_stat_to_zero() {
        let block = StatBlock::new();
        assert_eq!(block.stat(StatKind::Strength).base_value(), 0);
        assert_eq!(block.stat(StatKind::Strength).value(&block), 0);
    }

    #[test]
    pub fn stats_have_tickers() {
        assert_eq!(StatKind::Strength.ticker(), "STR");
    }

    #[test]
    pub fn stat_can_be_checked() {
        let mut block = StatBlock::new();
        assert!(!block.check(StatKind::Strength, 5));
        assert!(!block.check(StatKind::Strength, 6));
        block.mut_stat(StatKind::Strength).set_base_value(5);
        assert!(block.check(StatKind::Strength, 5));
        assert!(!block.check(StatKind::Strength, 6));
        // Transfers should be used when making checks.
        block.mut_stat(StatKind::Agility).set_base_value(10);
        assert!(block.check(StatKind::Strength, 6));
    }

    #[test]
    pub fn stats_increase_as_they_advance() {
        let mut block = StatBlock::new();
        block.mut_stat(StatKind::Strength).advance(50);
        assert_eq!(block.stat(StatKind::Strength).base_value(), 0);
        block.mut_stat(StatKind::Strength).advance(100);
        assert_eq!(block.stat(StatKind::Strength).base_value(), 1);
    }

    #[test]
    pub fn derives_max_hp_from_constitution() {
        let mut block = StatBlock::new();
        block.mut_stat(StatKind::Constitution).set_base_value(8);
        assert_eq!(block.max_hp(), 64);
    }

    #[test]
    pub fn checks_with_progress_advance_stats() {
        let mut block = StatBlock::new();
        assert!(!block.check_with_progression(
            StatKind::Strength,
            ProgressCheck {
                required: 1,
                base_progress: 100,
            },
        ));
        assert!(block.check_with_progression(
            StatKind::Strength,
            ProgressCheck {
                required: 1,
                base_progress: 1000,
            },
        ));
        assert!(block.check_with_progression(
            StatKind::Strength,
            ProgressCheck {
                required: 5,
                base_progress: 100,
            },
        ));
    }

    #[test]
    pub fn prints_tables() {
        let mut block = StatBlock::new();
        block.mut_stat(StatKind::Strength).set_base_value(5);
        assert_eq!(
            block.print_table(),
            String::from(
                "| Stat      | Base | Modified | Progress | Next Level |
| Strength  |    5 |        5 |        0 |        100 |
| Endurance |    0 |        1 |        0 |        100 |
| Agility   |    0 |        1 |        0 |        100 |
"
            ),
            "\nGot table:\n{}",
            block.print_table()
        );
    }
}
