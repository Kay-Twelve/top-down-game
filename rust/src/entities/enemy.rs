use godot::prelude::*;
use godot::classes::{CharacterBody2D, CollisionShape2D, ICharacterBody2D, RectangleShape2D};

use crate::entities::{FollowEntity, InteractiveEntity};

#[derive(GodotClass)]
#[class(init, base=CharacterBody2D)]
pub struct Enemy {
  _name: String,
  _life: f64,
  speed: f32,
  base: Base<CharacterBody2D>,
  _target_entity: Option<DynGd<Node2D, dyn InteractiveEntity>>
}

#[godot_api]
impl ICharacterBody2D for Enemy {
  fn ready(&mut self) {
    let mut collision_shape = CollisionShape2D::new_alloc();
    let mut shape = RectangleShape2D::new_gd();
    shape.set_size(Vector2 { x: 60., y: 60. });
    collision_shape.set_shape(Option::Some(shape).as_ref());
    self.base_mut().add_child(&collision_shape);
  }

  fn physics_process(&mut self, delta: f64) {
    self.follow_entity(delta);
  }
}

#[godot_api]
impl Enemy {
  #[func]
  pub fn new(_name: GString, _life: f64) -> Gd<Self> {
    Gd::from_init_fn(|base| {
      Self {
        _name: _name.into(),
        _life,
        speed: 40.,
        base,
        _target_entity: Option::None
      }
    })
  }
}

impl super::FollowEntity for Enemy {
  fn set_target_entity(&mut self, entity: DynGd<Node2D, dyn InteractiveEntity>) {
    self._target_entity = Option::Some(entity);
  }

  fn follow_entity(&mut self, delta: f64) {
    let entity_gd = self._target_entity.as_mut().unwrap();
    let entity: &Node2D = entity_gd.upcast_mut();
    let entity_pos = entity.get_position();
    let enemy_pos = self.base().get_position();
    let distance_target = Vector2 {
      x: entity_pos.x-enemy_pos.x,
      y: entity_pos.y-enemy_pos.y
    };

    if distance_target.x.abs()>150. || distance_target.y.abs()>150. {
      let x = if distance_target.x.round()==0. { 0. } else { (delta as f32)*self.speed*(if distance_target.x>0. { 1. } else { -1. }) };
      let y = if distance_target.y.round()==0. { 0. } else { (delta as f32)*self.speed*(if distance_target.y>0. { 1. } else { -1. }) };
      self.base_mut().move_and_collide(Vector2 { x, y });  
    }
  }
}

