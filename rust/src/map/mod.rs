use godot::prelude::*;
use godot::classes::{Area2D, IArea2D};

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct Map {
  base: Base<Area2D>
}

#[godot_api]
impl IArea2D for Map {
  fn init(base: Base<Area2D>) -> Self {
    Self {
      base
    }
  }
}