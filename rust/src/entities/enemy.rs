use godot::prelude::*;
use godot::classes::{CharacterBody2D, ICharacterBody2D};

use crate::entities::Attacking;
use crate::models;
use crate::models::hive_mind_system::HiveMindSystem;
use crate::models::motion_control_system::MotionControlSystem;

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
    pub idx: i16,
    _name: String,
    _life: f64,
    _state: EnemyStates,
    _attack_cooldown: models::timer::TimeManager,
    _cooldown_changing_direction: models::timer::TimeManager,
    _hive_mind_system: Option<Gd<HiveMindSystem>>,
    _motion_control_system: Gd<MotionControlSystem>,
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
          if *self._attack_cooldown>1.8 {
            self.attack(delta, &entity.clone());
            self._attack_cooldown.reset();
          }
        }
    }
}

#[godot_api]
impl Enemy {
    #[func]
    pub fn new(_name: GString, _life: f64, _hive_mind_system: Gd<HiveMindSystem>) -> Gd<Self> {
        Gd::from_init_fn(|base| Self {
            idx: _hive_mind_system.bind().entities.len() as i16,
            _name: _name.into(),
            _life,
            _state: EnemyStates::Inactive,
            _attack_cooldown: models::timer::TimeManager::new(),
            _cooldown_changing_direction: models::timer::TimeManager::new(),
            _hive_mind_system: Option::Some(_hive_mind_system.clone()),
            _motion_control_system: MotionControlSystem::new(240.),
            base,
        })
    }

    pub fn on_retrieve_behavioral_refs(&mut self) {
        if let Option::Some(hive_mind_system) = &self._hive_mind_system {
            if (hive_mind_system.bind().entities.len()-1) as i16 > self.idx {
                if *self._cooldown_changing_direction>=1.8 {
                    let entity = hive_mind_system.bind().entities.at((self.idx+1) as usize);
                    let constant_speed = self._motion_control_system.bind().get_constant_speed();
                    let t = entity.bind()._motion_control_system.bind().get_constant_speed();
                    if (constant_speed-t).length()<80. {
                        let normalized_pos = self._motion_control_system.bind().get_constant_speed().normalized_or_zero().rotated(5.);
                        self._motion_control_system.bind_mut().set_constant_speed(normalized_pos);
                    }
                }
            }
            return;
        }
    }

    pub fn set_position(&mut self, pos: Vector2) {
        self.base_mut().set_position(pos);
    }
}

#[godot_dyn]
impl super::InteractiveEntity for Enemy {}

impl super::FollowEntity for Enemy {
    fn set_target_entity(&mut self, entity: DynGd<Node2D, dyn InteractiveEntity>) {
        self._state = EnemyStates::FollowEntity(entity);
    }

    fn follow_entity(&mut self, delta: f64, entity_gd: &DynGd<Node2D, dyn InteractiveEntity>) {
        let entity: &Node2D = entity_gd.upcast_ref();
        let distance_target = (entity.get_position()-self.base().get_position()).round();
        if distance_target.length() > 1000. {
            if let EnemyStates::FollowEntity(entity) = &self._state { self._state = EnemyStates::Attacking(entity.to_owned()); } 
            else { self._state = EnemyStates::Inactive };
        } else if distance_target.length() > 200. {
            self._cooldown_changing_direction.update(delta);
            let new_velocity = (delta as f32)*(if *self._cooldown_changing_direction>=2. {
                self._cooldown_changing_direction.reset();
                self._motion_control_system.bind_mut().set_constant_speed(distance_target.normalized_or_zero())
            } else {
                self._motion_control_system.bind().get_constant_speed()
            });
            self.base_mut().move_and_collide(new_velocity);
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

