use godot::prelude::*;
use godot::classes::{CharacterBody2D, ICharacterBody2D};

use crate::models::{self, entities, systems, timer};
use systems::{
    hive_mind_system::HiveMindSystem,
    motion_control_system::MotionControlSystem,
    decision_making_system::{DecisionSystem, BehavioralStates, TypeOfPersecution}
};
use entities::{
    InteractiveEntity,
    Attacking, 
    FollowEntity,
    EntityCreator
};

#[derive(Default)]
pub enum StatesOfEnemyAction<TargetEntity = DynGd<Node2D, dyn InteractiveEntity>> {
    #[default] Inactive,
    Attacking(TargetEntity),
}

#[derive(GodotClass)]
#[class(no_init, base=CharacterBody2D)]
pub struct Enemy {
    pub idx: i16,
    _name: String,
    _life: f64,
    _decision: DecisionSystem<StatesOfEnemyAction, 1, 1, DynGd<Node2D, dyn InteractiveEntity>>,
    _attack_cooldown: timer::TimeManager,
    _hive_mind_system: Option<Gd<HiveMindSystem>>,
    _motion_control_system: Gd<MotionControlSystem>,
    base: Base<CharacterBody2D>,
}

#[godot_api]
impl ICharacterBody2D for Enemy {
    fn ready(&mut self) {
        self.base_mut().add_child(&EntityCreator::add_collision_shape(Vector2 { 
          x: 60., 
          y: 60. 
        }));
    }

    fn physics_process(&mut self, delta: f32) {
        self._decision._locomotion._cooldown.update(delta);
        if let (
            BehavioralStates::Persecution(TypeOfPersecution::FollowEntity), 
            Option::Some(entity)
        ) = (
            &self._decision._locomotion._state, 
            &self._decision._locomotion._target_entity
        ) {
            self.follow_entity(delta, &entity.clone());
        } else if let BehavioralStates::RePositioning(time, pos) = &self._decision._locomotion._state {
            let new_velocity = delta*(self._motion_control_system.bind().get_constant_speed());
            self.base_mut().move_and_collide(new_velocity);
        }

        if *self._decision._locomotion._cooldown>=7.5 {
            self._decision._locomotion._cooldown.reset();
        }

    }
}

#[godot_api]
impl Enemy {
    #[func]
    pub fn new(_name: GString, _life: f64, _hive_mind_system: Gd<HiveMindSystem>) -> Gd<Self> {
        let initial_velocity = Vector2 { x: 0.5, y: 0.5 };
        Gd::from_init_fn(|base| Self {
            idx: _hive_mind_system.bind().entities.len() as i16,
            _name: _name.into(),
            _life,
            _decision: DecisionSystem::default(),
            _attack_cooldown: timer::TimeManager::default(),
            _hive_mind_system: Option::Some(_hive_mind_system.clone()),
            _motion_control_system: MotionControlSystem::new(240., initial_velocity),
            base,
        })
    }

    pub fn on_retrieve_behavioral_refs(&mut self, delta: f32) {
        if let Option::Some(hive_mind_system) = &self._hive_mind_system {
            if (hive_mind_system.bind().entities.len()-1) as i16 > self.idx {
                if let BehavioralStates::Persecution(_) = &self._decision._locomotion._state {
                    if *self._decision._locomotion._cooldown>=1.8 {
                        let entity = hive_mind_system.bind().entities.at((self.idx+1) as usize);
                        let constant_speed = self._motion_control_system.bind().get_constant_speed();
                        let t = entity.bind()._motion_control_system.bind().get_constant_speed();
                        if (constant_speed-t).length()<80. {
                            self._decision._locomotion.save_previous_state();
                            self._decision._locomotion._state = BehavioralStates::RePositioning(2., timer::TimeManager::default());
                            let new_velocity = (self._motion_control_system.bind().get_constant_speed().normalized_or_zero()).rotated(1.);
                            self._motion_control_system.bind_mut().set_constant_speed(new_velocity);
                        }
                    };    
                } else if let &mut BehavioralStates::RePositioning(ref duration, ref mut timer) = &mut self._decision._locomotion._state {
                    timer.update(delta);                    
                    if timer.current>=*duration {
                        timer.reset();
                        self._decision._locomotion.restore_to_previous_state();
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
impl InteractiveEntity for Enemy {}

impl FollowEntity for Enemy {
    fn set_a_target_for_pursuit(&mut self, entity: DynGd<Node2D, dyn InteractiveEntity>) {
        self._decision._locomotion.set_target_entity(
            BehavioralStates::Persecution(TypeOfPersecution::FollowEntity), 
            entity
        );
    }

    fn follow_entity(&mut self, delta: f32, entity_gd: &DynGd<Node2D, dyn InteractiveEntity>) {
        let entity: &Node2D = entity_gd.upcast_ref();
        let distance_target = (entity.get_position()-self.base().get_position()).round();
        if distance_target.length() > 200. {
            let new_velocity = (delta)*(if *self._decision._locomotion._cooldown>=2. {
                self._motion_control_system.bind_mut().set_constant_speed(distance_target.normalized_or_zero())
            } else {
                self._motion_control_system.bind().get_constant_speed()
            });
            self.base_mut().move_and_collide(new_velocity);
        } 
    }
}

impl Attacking for Enemy {
    fn attack(&mut self, _delta: f32, entity: &DynGd<Node2D, dyn InteractiveEntity>) {
        let mut new_attack = entities::attack::Attack::new(0.5, 4.);
        self.base_mut()
            .get_parent()
            .expect("The enemy entity has no parent")
            .add_child(&new_attack);
        new_attack.bind_mut().set_init_pos(self.base().get_position());
        new_attack.bind_mut().set_constant_speed(entity.get_position());
    }
}

