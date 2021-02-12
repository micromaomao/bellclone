use std::time::Instant;

use game_core::ec::{register_common_components, register_common_systems, DeltaTime};
use specs::{Dispatcher, DispatcherBuilder, World, WorldExt};

pub struct EcCtx {
  pub world: World,
  game_dispatch: Dispatcher<'static, 'static>,
  last_update: Instant,
}

impl EcCtx {
  pub fn new() -> EcCtx {
    let mut w = World::new();
    Self {
      world: w,
      game_dispatch: game_dispatch.build(),
      last_update: Instant::now(),
    }
  }

  pub fn update(&mut self) {
    let now = Instant::now();
    let dt = now.duration_since(self.last_update);
    self.last_update = now;
    let dt = DeltaTime::from_secs_f32(dt.as_secs_f32());
    *self.world.write_resource::<DeltaTime>() = dt;
    self.game_dispatch.dispatch(&self.world);
    self.world.maintain();
  }
}
