use godot::prelude::*;
use godot::classes::Node2D;
use crate::entities;

#[derive(GodotClass)]
#[class(init, base=Node2D)]
pub struct HiveMindSystem {
  pub entities: Array<Gd<entities::enemy::Enemy>>,
  pub positions: Array<Vector2>,
  base: Base<Node2D>
}

#[godot_api]
impl HiveMindSystem {
  pub fn new() -> Gd<Self> {
    Gd::from_init_fn(|base| {
      Self {
        entities: array![],
        positions: array![],
        base
      }
    })
  }

  #[signal]
  pub fn retrieve_behavioral_refs();

  pub fn on_retrieve_behavioral_refs(&mut self) {}

  pub fn add_entity(&mut self, entity: Gd<entities::enemy::Enemy>) {
    self.signals()
      .retrieve_behavioral_refs()
      .connect_other(&entity, entities::enemy::Enemy::on_retrieve_behavioral_refs);
    self.positions.push(entity.get_position());
    self.entities.push(&entity);
  }
}

#[godot_api]
impl INode2D for HiveMindSystem {
  fn ready(&mut self) {
    self.signals()
      .retrieve_behavioral_refs()
      .connect_self(Self::on_retrieve_behavioral_refs);
  }
}