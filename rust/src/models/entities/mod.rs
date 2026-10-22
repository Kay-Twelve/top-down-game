pub mod attack;

use godot::prelude::*;
use godot::classes::{CollisionShape2D, RectangleShape2D};

pub struct EntityCreator;

impl EntityCreator {
  pub fn add_collision_shape(size: Vector2) -> Gd<CollisionShape2D> {
    let mut collision_shape = CollisionShape2D::new_alloc();
    let mut shape = RectangleShape2D::new_gd();
    shape.set_size(size);
    collision_shape.set_shape(Option::Some(shape).as_ref());
    collision_shape
  }
}

pub trait InteractiveEntity: 'static {}

pub trait FollowEntity {
  fn set_a_target_for_pursuit(&mut self, entity: DynGd<Node2D, dyn InteractiveEntity>);
  fn follow_entity(&mut self, delta: f32, entity_gd: &DynGd<Node2D, dyn InteractiveEntity>);
}

pub trait Attacking {
  fn attack(&mut self, delta: f32, entity_gd: &DynGd<Node2D, dyn InteractiveEntity>);
}