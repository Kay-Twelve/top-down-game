use std::ops::Deref;

#[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct TimeManager {
  pub current: f32
}

impl TimeManager {
  pub fn update(&mut self, delta: f32) {
    self.current += delta;
  }

  pub fn reset(&mut self) {
    self.current = 0.0;
  }
}

impl Deref for TimeManager {
  type Target = f32;

  fn deref(&self) -> &Self::Target {
    &self.current
  }
}