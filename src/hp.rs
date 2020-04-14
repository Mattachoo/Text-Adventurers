pub struct HitPoints {
    current: i64,
    max: i64,
}

pub enum HitPointState {
    Full,
    Damaged,
    Depleted,
}

impl HitPoints {
    pub fn new(max: i64) -> HitPoints {
        if max <= 0 {
            panic!("Non-positive hit point maximums are disallowed")
        }
        HitPoints{current: max, max}
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

    fn set_current(&mut self, new_hp: i64) {
        self.current = new_hp;
        if self.current < 0 {
            self.current = 0;
        } else if self.current > self.max {
            self.current = self.max
        }
    }
}

