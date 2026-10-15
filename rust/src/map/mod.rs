use crate::entities::{self, FollowEntity, InteractiveEntity};

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

  fn ready(&mut self) {
    let player = self.base().get_node_as::<entities::player::Player>("../Player");
    let mut new_enemy = entities::enemy::Enemy::new("Test Enemy".into(), 80.);
    self.base_mut().add_child(&new_enemy);
    new_enemy.bind_mut().set_target_entity(player.into_dyn::<dyn InteractiveEntity>().upcast());
  }
}