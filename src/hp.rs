pub struct HitPoints {
    current: i64,
    max: i64,
}

#[derive(Debug, PartialEq)]
pub enum HitPointState {
    Full,
    Damaged,
    Depleted,
}

impl HitPoints {
    pub fn new(mut max: i64) -> HitPoints {
        if max <= 0 {
            max = 1
        }
        HitPoints { current: max, max }
    }

    pub fn current(&self) -> i64 {
        self.current
    }

    pub fn max(&self) -> i64 {
        self.max
    }

    pub fn state(&self) -> HitPointState {
        if self.current == self.max {
            HitPointState::Full
        } else if self.current == 0 {
            HitPointState::Depleted
        } else {
            HitPointState::Damaged
        }
    }

    // Negative damage is ignored.
    pub fn take_damage(&mut self, damage: i64) -> HitPointState {
        if damage < 0 {
            return self.state();
        }
        self.set_current(self.current - damage);
        self.state()
    }

    // Negative healing is expected: certain kinds of resistances can lead to it.
    pub fn heal(&mut self, health: i64) -> HitPointState {
        self.set_current(self.current + health);
        self.state()
    }

    // Guarantees state(&self) will return HitPointState::Full.
    pub fn heal_fully(&mut self) {
        self.current = self.max;
    }

    pub fn set_max(&mut self, mut new_max: i64) {
        if new_max <= 0 {
            new_max = 1
        }
        self.max = new_max;
        if self.current > self.max {
            self.current = self.max
        }
    }

    fn set_current(&mut self, new_hp: i64) {
        self.current = new_hp;
        if self.current < 0 {
            self.current = 0;
        } else if self.current > self.max {
            self.current = self.max
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn stores_max_hitpoints() {
        let hp = HitPoints::new(42);
        assert_eq!(hp.max, 42);
    }

    #[test]
    pub fn starts_at_max_hitpoints() {
        let hp = HitPoints::new(42);
        assert_eq!(hp.current, 42);
    }

    #[test]
    pub fn starts_in_full_hitpoint_state() {
        let hp = HitPoints::new(42);
        assert_eq!(hp.state(), HitPointState::Full)
    }

    #[test]
    pub fn clamps_zero_hp_to_one() {
        let hp = HitPoints::new(0);
        assert_eq!(hp.current, 1);
        assert_eq!(hp.max, 1);
    }

    #[test]
    pub fn clamps_negative_hp_to_one() {
        let hp = HitPoints::new(-5);
        assert_eq!(hp.current, 1);
        assert_eq!(hp.max, 1);
    }

    #[test]
    pub fn taking_damage_updates_current_hp() {
        let mut hp = HitPoints::new(100);
        hp.take_damage(20);
        assert_eq!(hp.current, 80);
        assert_eq!(hp.max, 100);
        hp.take_damage(45);
        assert_eq!(hp.current, 35);
        assert_eq!(hp.max, 100);
        hp.take_damage(35);
        assert_eq!(hp.current, 0);
        assert_eq!(hp.max, 100);
    }

    #[test]
    pub fn taking_damage_updates_state() {
        let mut hp = HitPoints::new(100);
        hp.take_damage(20);
        assert_eq!(hp.state(), HitPointState::Damaged);
        hp.take_damage(45);
        assert_eq!(hp.state(), HitPointState::Damaged);
        hp.take_damage(35);
        assert_eq!(hp.state(), HitPointState::Depleted);
    }

    #[test]
    pub fn negative_damage_does_nothing() {
        let mut hp = HitPoints::new(100);
        hp.take_damage(-20);
        assert_eq!(hp.current, 100);
        assert_eq!(hp.state(), HitPointState::Full);
    }

    #[test]
    pub fn zero_damage_does_nothing() {
        let mut hp = HitPoints::new(100);
        hp.take_damage(0);
        assert_eq!(hp.current, 100);
        assert_eq!(hp.state(), HitPointState::Full);
    }

    #[test]
    pub fn current_snaps_to_zero_on_massive_damage() {
        let mut hp = HitPoints::new(100);
        hp.take_damage(150);
        assert_eq!(hp.current, 0);
        assert_eq!(hp.state(), HitPointState::Depleted);
    }

    #[test]
    pub fn positive_healing_adjusts_hp() {
        let mut hp = HitPoints::new(100);
        hp.take_damage(50);
        hp.heal(5);
        assert_eq!(hp.current, 55);
        assert_eq!(hp.state(), HitPointState::Damaged);
    }

    #[test]
    pub fn positive_healing_caps_hp_to_max() {
        let mut hp = HitPoints::new(100);
        hp.take_damage(50);
        hp.heal(150);
        assert_eq!(hp.current, 100);
        assert_eq!(hp.state(), HitPointState::Full);
    }

    #[test]
    pub fn negative_healing_adjusts_hp() {
        let mut hp = HitPoints::new(100);
        hp.heal(-50);
        assert_eq!(hp.current, 50);
        assert_eq!(hp.state(), HitPointState::Damaged);
    }

    #[test]
    pub fn negative_healing_snaps_hp_to_zero() {
        let mut hp = HitPoints::new(100);
        hp.heal(-150);
        assert_eq!(hp.current, 0);
        assert_eq!(hp.state(), HitPointState::Depleted);
    }

    #[test]
    pub fn zero_healing_does_nothing() {
        let mut hp = HitPoints::new(100);
        hp.take_damage(50);
        hp.heal(0);
        assert_eq!(hp.current, 50);
        assert_eq!(hp.state(), HitPointState::Damaged);
    }

    #[test]
    pub fn heal_fully_restores_all_hp() {
        let mut hp = HitPoints::new(100);
        hp.take_damage(50);
        hp.heal_fully();
        assert_eq!(hp.current, 100);
        assert_eq!(hp.state(), HitPointState::Full);
    }

    #[test]
    pub fn max_hp_is_adjustable() {
        let mut hp = HitPoints::new(100);
        hp.set_max(150);
        assert_eq!(hp.current, 100);
        assert_eq!(hp.max, 150);
    }

    #[test]
    pub fn adjusted_max_hp_is_clamped_to_one() {
        let mut hp = HitPoints::new(100);
        hp.set_max(0);
        assert_eq!(hp.current, 1);
        assert_eq!(hp.max, 1);
    }

    #[test]
    pub fn adjusted_max_hp_limits_current_hp() {
        let mut hp = HitPoints::new(100);
        hp.set_max(50);
        assert_eq!(hp.current, 50);
        assert_eq!(hp.max, 50);
    }
}
