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