pub mod mass;

// Some units are represented as floating-point numbers. Their comparisons
// will use UNIT_PRECISION when determining equality.
// Units easily represented with an integer are represented as one.
pub const UNIT_PRECISION: f64 = 0.001;

pub enum UnitSystem {
    Metric,
    Imperial,
}

pub trait UnitDisplay {
    fn display_with_units(&self, units: UnitSystem) -> String;
}

