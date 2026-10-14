use std::ops::Deref;

pub struct TimeManager {
  pub current: f64
}

impl TimeManager {
  pub fn new() -> Self {
    Self {
      current: 0.0
    }
  }

  pub fn update(&mut self, delta: f64) {
    self.current += delta;
  }

  pub fn reset(&mut self) {
    self.current = 0.0;
  }
}

impl Deref for TimeManager {
  type Target = f64;

  fn deref(&self) -> &Self::Target {
    &self.current
  }
}