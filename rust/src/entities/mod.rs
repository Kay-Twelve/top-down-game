pub mod player;
pub mod enemy;

use godot::prelude::*;

pub trait FollowEntity {
  fn set_target_entity(&mut self, entity: DynGd<Node2D, dyn InteractiveEntity>);
  fn follow_entity(&mut self, delta: f64, entity_gd: &DynGd<Node2D, dyn InteractiveEntity>);
}

pub trait Attacking {
  fn attack(&mut self, delta: f64, entity_gd: &DynGd<Node2D, dyn InteractiveEntity>);
}

pub trait InteractiveEntity: 'static {}