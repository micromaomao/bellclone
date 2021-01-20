use specs::{World, WorldExt};

pub mod components;
pub mod systems;

pub fn register_common_components(w: &mut World) {
  w.register::<components::transform::WorldSpaceTransform>();
}
