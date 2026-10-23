use crate::models::timer;

#[derive(Default, Debug, Copy, Clone)]
pub enum TypeOfPersecution {
  #[default] Collectively,
  FollowEntity
}

#[derive(Default, Debug, Clone, Copy)]
pub enum BehavioralStates { 
  #[default] Inactive,
  Idle(f32),
  RePositioning(f32, timer::TimeManager),
  Persecution(TypeOfPersecution)
}

#[derive(Default, Debug, Copy, Clone)]
pub enum UpcomingStates<T, const N: usize> {
  #[default] Inactive,
  List([T; N])
}

pub struct StateOfDecisionMaking<T, const N: usize, TargetEntity> {
  pub _state: T,
  pub _target_entity: Option<TargetEntity>,
  pub _upcoming_states: UpcomingStates<T, N>,
  pub _cooldown: timer::TimeManager
}

impl<T, const N: usize, TargetEntity> Default for StateOfDecisionMaking<T, N, TargetEntity> where  T: Default {
  fn default() -> Self {
      Self {
        _state: T::default(),
        _target_entity: Option::None,
        _upcoming_states: UpcomingStates::Inactive,
        _cooldown: timer::TimeManager::default()
      }
  }
}

impl<T, TargetEntity, const N: usize> StateOfDecisionMaking<T, N, TargetEntity> {
  pub fn set_target_entity(&mut self, new_state: T, entity: TargetEntity) {
    self._state = new_state;
    self._target_entity = Option::Some(entity);
  }
}

impl<TargetEntity, const N: usize> StateOfDecisionMaking<BehavioralStates, N, TargetEntity> {
  pub fn save_previous_state(&mut self) {
    if N>=1 {
      if let UpcomingStates::List(mut list) = self._upcoming_states {
        list[0] = self._state;
      } else {
        let mut list = [BehavioralStates::Inactive; N];
        list[0] = self._state;
        self._upcoming_states = UpcomingStates::List(list);
      };
    }
  }

  pub fn restore_to_previous_state(&mut self) -> BehavioralStates {
    let mut old_state: BehavioralStates = BehavioralStates::Inactive;
    if let UpcomingStates::List(list) = self._upcoming_states { 
      let mut i = 0;
      loop {
        if let BehavioralStates::Inactive = list[i] { break; } else {
          old_state = self._state;
          self._state = list[i];
        };
        i += 1;
        if i>=N { break; }
      }
    }
    old_state
  }
}

pub struct DecisionSystem<StatesOfAction, const NL: usize, const NA: usize, TargetEntity> where StatesOfAction: Default {
  pub _locomotion: StateOfDecisionMaking<BehavioralStates, NL, TargetEntity>,
  pub _action: StateOfDecisionMaking<StatesOfAction, NA, TargetEntity>
}

impl<StatesOfAction, const NL: usize, const NA: usize, TargetEntity> Default for DecisionSystem<StatesOfAction, NL, NA, TargetEntity> where StatesOfAction: Default {
  fn default() -> Self {
    Self {
      _locomotion: StateOfDecisionMaking::default(),
      _action: StateOfDecisionMaking::default()
    }
  }
} 

