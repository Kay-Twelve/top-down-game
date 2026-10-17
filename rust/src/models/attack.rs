use godot::prelude::*;
use godot::classes::{Area2D, IArea2D};

#[derive(GodotClass)]
#[class(init, base=Area2D)]
pub struct Attack {
  _damage: f32,
  _speed: f32,
  _constant_speed: Vector2,
  base: Base<Area2D>
}

#[godot_api]
impl Attack {
  #[func]
  pub fn new(_damage: f32, _speed: f32) -> Gd<Self> {
    Gd::from_init_fn(|base| {
      Self {
        _damage,
        _speed,
        _constant_speed: Vector2::ZERO, 
        base
      }
    })
  }

  pub fn set_init_pos(&mut self, _pos: Vector2) {
    self.base_mut().set_position(_pos);
  }

  pub fn set_constant_speed(&mut self, _final_pos: Vector2) {
    let pos = self.base().get_position();
    let target_pos_vector = _final_pos-pos;
    godot_print!("{:?}", target_pos_vector.normalized_or_zero());
    self._constant_speed = target_pos_vector.normalized_or_zero()*self._speed;
  }
}

#[godot_api]
impl IArea2D for Attack {
  fn ready(&mut self) {
    self.base_mut().add_child(&super::entity::EntityCreator::add_collision_shape(Vector2 {
      x: 30.,
      y: 30.
    }));
  }

  fn physics_process(&mut self, delta: f32) {
    let new_pos = (self.base().get_position()+self._constant_speed);
    self.base_mut().set_position(new_pos);
  }
}