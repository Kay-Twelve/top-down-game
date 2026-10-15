use crate::entities::InteractiveEntity;
use crate::models;

use godot::prelude::*;
use godot::classes::{ICharacterBody2D, CharacterBody2D, Input};

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Player {
  #[export]
  speed: f32,
  base: Base<CharacterBody2D>,
  map: Option<Gd<crate::map::Map>>,
  race_timer: models::timer::TimeManager
}

impl Player {
  fn direction(&self) -> Vector2 {
    let input = Input::singleton();
    input.get_vector("ui_left", "ui_right", "ui_up", "ui_down")
  }
}

#[godot_dyn]
impl InteractiveEntity for Player {}

#[godot_api]
impl ICharacterBody2D for Player {
  fn init(base: Base<CharacterBody2D>) -> Self {
    Self {
      speed: 10.,
      base,
      map: Option::None,
      race_timer: models::timer::TimeManager::new()
    }
  }

  fn ready(&mut self) {
    self.map = Option::Some(self.base().get_node_as::<crate::map::Map>("../Map"));
  }

  fn physics_process(&mut self, delta: f64) {
    let map: &Gd<crate::map::Map> = self.map
      .as_ref()
      .expect("There is no node named {Map} in scene");

    let binding = self.base();
    if !map.bind().base().overlaps_body(binding.to_godot()) {
      godot_print!("Fell off the platform!");
    }

    let direction = self.direction();
    if direction.x!=0. || direction.y!=0. {
      self.race_timer.update(delta);

      let mut x = self.speed;
      let mut y = self.speed;

      if *self.race_timer>=5. {
        x += 7.;
        y += 4.;
      }

      x *= direction.x;
      y *= direction.y;

      self.base_mut().move_and_collide(Vector2 { x, y });
    } else {
      self.race_timer.reset();
    }
  }
}