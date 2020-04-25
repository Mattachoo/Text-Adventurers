use crate::unit::UnitDisplay;
use crate::unit::UnitSystem;
use crate::unit::UNIT_PRECISION;
use std::ops;

#[derive(Copy, Clone, Debug)]
pub struct Temperature {
    kelvin: f64,
}

// Negative temperatures (i.e. below absolute zero) as well as NaNs and
// infinity are disallowed.
// If close enough to absolute zero to constitute equality, they will be left
// alone.
// Otherwise, the program will crash.
impl Temperature {
    pub const fn absolute_zero() -> Temperature {
        Temperature{kelvin: 0.0}
    }

    pub fn from_kelvin(kelvin: f64) -> Temperature {
        let t = Temperature{kelvin};
        t.check_validity();
        t
    }

    pub fn from_celsius(celsius: f64) -> Temperature {
        let t = Temperature{kelvin: celsius + 273.15};
        t.check_validity();
        t
    }

    pub fn from_farenheit(farenheit: f64) -> Temperature {
        Temperature::from_celsius((farenheit - 32.0) * 5.0 / 9.0)
    }

    pub fn to_kelvin(&self) -> f64 {
        self.kelvin
    }

    pub fn to_celsius(&self) -> f64 {
        self.to_kelvin() - 273.15
    }

    pub fn to_farenheit(&self) -> f64 {
        self.to_celsius() * 9.0 / 5.0 + 32.0
    }

    pub fn approx_equal(&self, other: &Temperature) -> bool {
        (self.kelvin - other.kelvin).abs() < UNIT_PRECISION
    }

    fn check_validity(&self) {
        if !self.kelvin.is_finite() {
            panic!("nonfinite temperatures are disallowed");
        }
        if self.kelvin < 0.0 && !self.approx_equal(&Temperature::absolute_zero()) {
            panic!("negative temperatures are disallowed");
        }
    }
}

impl UnitDisplay for Temperature {
    fn display_with_units(&self, units: UnitSystem) -> String {
        match units {
            UnitSystem::Metric => format!("{:.1} °C", self.to_celsius()),
            UnitSystem::Imperial => format!("{:.1} °F", self.to_farenheit()),
        }
    }
}

impl ops::Add<Temperature> for Temperature {
    type Output = Temperature;

    fn add(self, rhs: Temperature) -> Temperature {
        // Repeated addition of slight negatives should not break anything.
        if self.approx_equal(&Temperature::absolute_zero()) {
            return rhs
        } else if rhs.approx_equal(&Temperature::absolute_zero()) {
            return self
        }
        Temperature::from_kelvin(self.kelvin + rhs.kelvin)
    }
}

impl ops::Sub<Temperature> for Temperature {
    type Output = Temperature;

    fn sub(self, rhs: Temperature) -> Temperature {
        if rhs.approx_equal(&Temperature::absolute_zero()) {
            return self
        }
        Temperature::from_kelvin(self.kelvin - rhs.kelvin)
    }
}

impl ops::Mul<f64> for Temperature {
    type Output = Temperature;

    fn mul(self, rhs: f64) -> Temperature {
        // To avoid exacerbating a slight negative, we must simply return zero.
        if self.approx_equal(&Temperature::absolute_zero()) {
            return Temperature::absolute_zero();
        }
        Temperature::from_kelvin(self.kelvin * rhs)
    }
}

#[cfg(test)]
mod tests {
    use crate::unit::UNIT_PRECISION;
    use super::*;

    #[test]
    fn represents_absolute_zero() {
        let a = Temperature::from_kelvin(0.0);
        let b = Temperature::from_kelvin(0.0);
        assert!(a.approx_equal(&a));
        assert!(b.approx_equal(&b));
        assert!(a.approx_equal(&b));
        assert!(b.approx_equal(&a));
    }

    #[test]
    fn represents_near_zeroes() {
        Temperature::from_kelvin(-0.000001);
    }

    #[test]
    #[should_panic]
    fn disallows_negative_kelvin() {
        Temperature::from_kelvin(-1.0);
    }

    #[test]
    #[should_panic]
    fn disallows_negative_farenheit() {
        // Just past absolute zero.
        Temperature::from_kelvin(-460.0);
    }

    #[test]
    #[should_panic]
    fn disallows_negative_celsius() {
        // Just past absolute zero.
        Temperature::from_celsius(-274.0);
    }

    #[test]
    fn supports_addition() {
        assert!((Temperature::from_kelvin(5.0)
                 + Temperature::from_kelvin(25.0)).approx_equal(
                     &Temperature::from_kelvin(30.0)));
    }

    #[test]
    fn safely_adds_slight_negatives() {
        let mut base = Temperature::from_kelvin(0.1);
        let slight_negative = Temperature::from_kelvin(UNIT_PRECISION * -0.5);
        for _i in 1..1000 {
            base = base + slight_negative;
        }
    }

    #[test]
    fn supports_subtraction() {
        assert!((Temperature::from_kelvin(25.0)
                 - Temperature::from_kelvin(5.0)).approx_equal(
                     &Temperature::from_kelvin(20.0)));
    }

    #[test]
    #[should_panic]
    fn disallows_negative_subtraction() {
        let _ = Temperature::from_kelvin(20.0) - Temperature::from_kelvin(25.0);
    }

    #[test]
    fn supports_multiplication() {
        let product = Temperature::from_kelvin(1.0) * 100.0;
        assert!(product.approx_equal(&Temperature::from_kelvin(100.0)),
                "{:?} ~= 100K",
                product);
    }

    #[test]
    fn supports_slight_negative_multiplication() {
        let product = Temperature::from_kelvin(-0.000001) * 100.0;
        assert!(product.approx_equal(&Temperature::absolute_zero()),
                "{:?} ~= 0K",
                product);
    }

    #[test]
    #[should_panic]
    fn disallows_major_negative_multiplication() {
        let _ = Temperature::from_kelvin(5.0) * -10.0;
    }

    #[test]
    fn converts_to_kelvin() {
        assert!((Temperature::from_kelvin(3.1415).to_kelvin()
                 - 3.1415).abs() < UNIT_PRECISION);
    }

    #[test]
    fn converts_from_farenheit() {
        let freezing_farenheit = Temperature::from_farenheit(32.0);
        let freezing_kelvin = Temperature::from_kelvin(273.15);
        assert!(freezing_farenheit.approx_equal(&freezing_kelvin),
                "{:?} ~= {:?}",
                freezing_farenheit,
                freezing_kelvin);
        let boiling_farenheit = Temperature::from_farenheit(212.0);
        let boiling_kelvin = Temperature::from_kelvin(373.15);
        assert!(boiling_farenheit.approx_equal(&boiling_kelvin),
                "{:?} ~= {:?}",
                boiling_farenheit,
                boiling_kelvin);
    }

    #[test]
    fn converts_to_farenheit() {
        let freezing = Temperature::from_kelvin(273.15);
        assert!((freezing.to_farenheit() - 32.0).abs() < UNIT_PRECISION,
                 "{} ~= 32.0",
                 freezing.to_farenheit());
        let boiling = Temperature::from_kelvin(373.15);
        assert!((boiling.to_farenheit() - 212.0).abs() < UNIT_PRECISION,
                 "{} ~= 212.0",
                 boiling.to_farenheit());
    }

    #[test]
    fn converts_from_celsius() {
        let freezing_celsius = Temperature::from_celsius(0.0);
        let freezing_kelvin = Temperature::from_kelvin(273.15);
        assert!(freezing_celsius.approx_equal(&freezing_kelvin),
                "{:?} ~= {:?}",
                freezing_celsius,
                freezing_kelvin);
        let boiling_celsius = Temperature::from_celsius(100.0);
        let boiling_kelvin = Temperature::from_kelvin(373.15);
        assert!(boiling_celsius.approx_equal(&boiling_kelvin),
                "{:?} ~= {:?}",
                boiling_celsius,
                boiling_kelvin);
    }

    #[test]
    fn converts_to_celsius() {
        let freezing = Temperature::from_kelvin(273.15);
        assert!((freezing.to_celsius() - 0.0).abs() < UNIT_PRECISION,
                 "{} ~= 0.0",
                 freezing.to_celsius());
        let boiling = Temperature::from_kelvin(373.15);
        assert!((boiling.to_celsius() - 100.0).abs() < UNIT_PRECISION,
                 "{} ~= 100.0",
                 boiling.to_celsius());
    }

    #[test]
    fn implements_display() {
        assert_eq!(Temperature::from_farenheit(67.6)
                   .display_with_units(UnitSystem::Imperial),
                   String::from("67.6 °F"));
        assert_eq!(Temperature::from_farenheit(98.6)
                   .display_with_units(UnitSystem::Imperial),
                   String::from("98.6 °F"));
        assert_eq!(Temperature::from_celsius(0.0)
                   .display_with_units(UnitSystem::Metric),
                   String::from("0.0 °C"));
        assert_eq!(Temperature::from_celsius(21.4)
                   .display_with_units(UnitSystem::Metric),
                   String::from("21.4 °C"));
    }
}
