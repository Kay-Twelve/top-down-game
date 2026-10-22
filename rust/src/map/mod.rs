
use crate::{models, entities};

use models::{
  systems::hive_mind_system::HiveMindSystem,
  entities::{InteractiveEntity, FollowEntity}
};

use godot::prelude::*;
use godot::classes::{Area2D, IArea2D};

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct Map {
  _hive_mind_system: Gd<HiveMindSystem>,
  base: Base<Area2D>
}

#[godot_api]
impl IArea2D for Map {
  fn init(base: Base<Area2D>) -> Self {
    Self {
      _hive_mind_system: HiveMindSystem::new(),
      base
    }
  }

  fn ready(&mut self) {
    let player = self.base().get_node_as::<entities::player::Player>("../Player");

    let (num_of_enemies, mut i) = (7, 0);
    let (mut x, mut y): (f32, f32) = (20., 20.);
    loop {
      let mut new_enemy = self.create_new_enemy();
      new_enemy.bind_mut().set_position(Vector2 { x, y });
      new_enemy.bind_mut().set_a_target_for_pursuit(player.clone().into_dyn::<dyn InteractiveEntity>().upcast());
      self._hive_mind_system.bind_mut()
        .add_entity(new_enemy);
      x += 120.;
      y += 120.;
      i += 1;
      if i>num_of_enemies-1 { break }
    };
  }

  fn physics_process(&mut self, delta: f32) {
    self._hive_mind_system.bind_mut()
      .signals()
      .retrieve_behavioral_refs()
      .emit(delta);
  }
}

#[godot_api]
impl Map {
  fn create_new_enemy(&mut self) -> Gd<entities::enemy::Enemy> {
    let new_enemy = entities::enemy::Enemy::new("Test Enemy".into(), 80., self._hive_mind_system.clone().upcast());
    self.base_mut().add_child(&new_enemy);
    new_enemy
  }
}