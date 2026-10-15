pub mod player;
pub mod enemy;

use godot::prelude::*;

pub trait FollowEntity {
  fn set_target_entity(&mut self, entity: DynGd<Node2D, dyn InteractiveEntity>);
  fn follow_entity(&mut self);
}

pub trait InteractiveEntity: 'static {}