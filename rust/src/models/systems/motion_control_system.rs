use godot::prelude::*;
use godot::classes::{RefCounted, IRefCounted};

#[derive(GodotClass)]
#[class(base=RefCounted)]
pub struct MotionControlSystem {
  pub _speed: f32,
  pub _constant_speed: Vector2,
  base: Base<RefCounted>
}

#[godot_api]
impl IRefCounted for MotionControlSystem {
  fn init(base: Base<RefCounted>) -> Self {
    Self {
      _speed: 0.,
      _constant_speed: Vector2::ZERO,
      base
    }
  }
}

#[godot_api]
impl MotionControlSystem {
  pub fn new(_speed: f32, mut _constant_speed: Vector2) -> Gd<Self> {
    _constant_speed = _constant_speed.normalized_or_zero()*_speed;
    Gd::from_init_fn(|base| {
      Self {
        _speed,
        _constant_speed,
        base
      }
    })
  }

  pub fn get_constant_speed(&self) -> Vector2 {
    self._constant_speed
  }

  pub fn set_constant_speed(&mut self, normalized_vel: Vector2) -> Vector2 {
    self._constant_speed = normalized_vel*self._speed;
    self._constant_speed
  }
}
