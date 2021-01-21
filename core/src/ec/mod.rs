use specs::{World, WorldExt};

pub mod components;
pub mod systems;

#[derive(Debug, Clone, Copy, Default)]
pub struct DeltaTime(pub f32);

impl DeltaTime {
  pub fn as_secs_f32(&self) -> f32 {
    self.0 / 1000f32
  }
}

pub fn register_common_components(w: &mut World) {
  w.register::<components::transform::WorldSpaceTransform>();
  w.register::<components::player::PlayerComponent>();
}
