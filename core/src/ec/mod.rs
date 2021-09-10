use specs::{DispatcherBuilder, World, WorldExt};

pub mod components;
pub mod systems;

const DELTA_TIME_MAX: f32 = 0.3f32;

#[derive(Debug, Clone, Copy, Default)]
pub struct DeltaTime(f32);

impl DeltaTime {
  pub fn from_secs_f32(secs: f32) -> Self {
    DeltaTime(secs.min(DELTA_TIME_MAX))
  }
  pub fn as_secs_f32(&self) -> f32 {
    self.0
  }
}

pub fn register_common_components(w: &mut World) {
  w.register::<components::transform::WorldSpaceTransform>();
  w.register::<components::player::PlayerComponent>();
  w.register::<components::EntityId>();
  w.register::<components::physics::Velocity>();
  w.register::<components::physics::Gravity>();
  w.register::<components::bell::BellComponent>();
}

pub fn register_common_systems(dispatch: &mut DispatcherBuilder) {
  dispatch.add(systems::physics::GravitySystem, "gravity_system", &[]);
  dispatch.add(
    systems::physics::VelocitySystem,
    "velocity_system",
    &["gravity_system"],
  );
  dispatch.add(
    systems::player::PlayerSystem,
    "player_system",
    &["velocity_system"],
  );
}
