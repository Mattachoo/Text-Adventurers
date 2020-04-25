use crate::unit::UnitDisplay;
use crate::unit::UnitSystem;
use std::ops;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Mass {
    grams: i64,
}

impl Mass {
    pub fn from_grams(grams: i64) -> Mass {
        Mass { grams }
    }

    pub fn to_grams(&self) -> i64 {
        self.grams
    }

    pub fn to_pounds(&self) -> f64 {
        self.grams as f64 * 0.000_453_592_37
    }
}

impl UnitDisplay for Mass {
    fn display_with_units(&self, units: UnitSystem) -> String {
        match units {
            UnitSystem::Metric => format!("{}g", self.grams),
            UnitSystem::Imperial => format!("{:.3}lb", self.to_pounds()),
        }
    }
}

impl ops::Add<Mass> for Mass {
    type Output = Mass;

    fn add(self, rhs: Mass) -> Mass {
        Mass {
            grams: self.grams + rhs.grams,
        }
    }
}

impl ops::Mul<i64> for Mass {
    type Output = Mass;

    fn mul(self, rhs: i64) -> Mass {
        if rhs <= 0 {
            panic!("zero and negative mass are not allowed")
        }
        Mass {
            grams: self.grams * rhs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn tracks_mass() {
        assert_eq!(Mass::from_grams(5), Mass { grams: 5 });
        assert_eq!(Mass::from_grams(5), Mass::from_grams(5));
        assert_eq!(Mass::from_grams(42).to_grams(), 42);
        assert_ne!(Mass::from_grams(1), Mass::from_grams(2));
    }

    #[test]
    pub fn implements_addition() {
        assert_eq!(
            Mass::from_grams(42) + Mass::from_grams(0),
            Mass::from_grams(42)
        );
        assert_eq!(
            Mass::from_grams(1) + Mass::from_grams(41),
            Mass::from_grams(42)
        );
    }

    #[test]
    pub fn implements_multiplication() {
        assert_eq!(Mass::from_grams(5) * 4, Mass::from_grams(20));
        assert_eq!(Mass::from_grams(42) * 2, Mass::from_grams(84));
    }

    #[test]
    pub fn displays_grams() {
        assert_eq!(
            Mass::from_grams(1000).display_with_units(UnitSystem::Metric),
            "1000g"
        );
    }

    #[test]
    pub fn displays_pounds() {
        assert_eq!(
            Mass::from_grams(1000).display_with_units(UnitSystem::Imperial),
            "0.454lb"
        );
    }
}
