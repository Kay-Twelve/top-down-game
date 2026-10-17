use godot::prelude::*;
use godot::classes::{CharacterBody2D, ICharacterBody2D};

use crate::entities::Attacking;
use crate::models;
use super::{FollowEntity, InteractiveEntity};

#[derive(Default)]
pub enum EnemyStates<TargetEntity = DynGd<Node2D, dyn InteractiveEntity>> {
    #[default]
    Inactive,
    FollowEntity(TargetEntity),
    Attacking(TargetEntity),
}

#[derive(GodotClass)]
#[class(init, base=CharacterBody2D)]
pub struct Enemy {
    _name: String,
    _life: f64,
    speed: f32,
    _state: EnemyStates,
    _attack_cooldown: models::timer::TimeManager,
    base: Base<CharacterBody2D>,
}

#[godot_api]
impl ICharacterBody2D for Enemy {
    fn ready(&mut self) {
        self.base_mut().add_child(&models::entity::EntityCreator::add_collision_shape(Vector2 { 
          x: 60., 
          y: 60. 
        }));
    }

    fn physics_process(&mut self, delta: f64) {
        if let EnemyStates::FollowEntity(entity) = &self._state {
            self.follow_entity(delta, &entity.clone());
        } else if let EnemyStates::Attacking(entity) = &self._state {
          self._attack_cooldown.update(delta);
          if *self._attack_cooldown>2.5 {
            self.attack(delta, &entity.clone());
            self._attack_cooldown.reset();
          }
        }
    }
}

#[godot_api]
impl Enemy {
    #[func]
    pub fn new(_name: GString, _life: f64) -> Gd<Self> {
        Gd::from_init_fn(|base| Self {
            _name: _name.into(),
            _life,
            speed: 40.,
            _state: EnemyStates::Inactive,
            _attack_cooldown: models::timer::TimeManager::new(),
            base,
        })
    }
}

impl super::FollowEntity for Enemy {
    fn set_target_entity(&mut self, entity: DynGd<Node2D, dyn InteractiveEntity>) {
        self._state = EnemyStates::FollowEntity(entity);
    }

    fn follow_entity(&mut self, delta: f64, entity_gd: &DynGd<Node2D, dyn InteractiveEntity>) {
        let entity: &Node2D = entity_gd.upcast_ref();
        let entity_pos = entity.get_position();
        let enemy_pos = self.base().get_position();
        let distance_target = Vector2 {
            x: entity_pos.x - enemy_pos.x,
            y: entity_pos.y - enemy_pos.y,
        };

        if distance_target.x.abs() > 300. || distance_target.y.abs() > 300. {
          if let EnemyStates::FollowEntity(entity) = &self._state { 
            self._state = EnemyStates::Attacking(entity.to_owned());
          } else { 
            self._state = EnemyStates::Inactive 
          };
        } else if distance_target.x.abs() > 150. || distance_target.y.abs() > 150. {
          let x = if distance_target.x.round() == 0. { 0. } else { (delta as f32) * self.speed * (if distance_target.x > 0. { 1. } else { -1. }) };
          let y = if distance_target.y.round() == 0. { 0. } else { (delta as f32) * self.speed * (if distance_target.y > 0. { 1. } else { -1. }) };
          self.base_mut().move_and_collide(Vector2 { x, y });
        }
    }
}

impl super::Attacking for Enemy {
    fn attack(&mut self, _: f64, entity: &DynGd<Node2D, dyn InteractiveEntity>) {
        let mut new_attack = models::attack::Attack::new(0.5, 4.);
        self.base_mut()
            .get_parent()
            .expect("The enemy entity has no parent")
            .add_child(&new_attack);
        new_attack.bind_mut().set_init_pos(self.base().get_position());
        new_attack.bind_mut().set_constant_speed(entity.get_position());
    }
}
