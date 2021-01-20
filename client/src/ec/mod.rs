use game_core::ec::register_common_components;
use specs::{Dispatcher, DispatcherBuilder};
use specs::{World, WorldExt};

use crate::render::DrawingCtx;

pub mod components;
pub mod systems;

pub struct EcCtx {
  pub world: World,
  render_dispatch: Dispatcher<'static, 'static>,
}

impl EcCtx {
  pub fn new() -> Self {
    let mut world = World::new();
    register_common_components(&mut world);
    register_client_components(&mut world);
    let mut render_dispatch = DispatcherBuilder::new();
    render_dispatch.add_thread_local(systems::draw_debug::DrawDebug);
    world.maintain();
    EcCtx {
      world,
      render_dispatch: render_dispatch.build(),
    }
  }

  pub fn update(&mut self) {
    self.world.maintain();
  }

  pub fn render(&mut self, dctx: DrawingCtx) {
    self.world.insert(dctx);
    self.render_dispatch.dispatch(&self.world);
  }
}

fn register_client_components(w: &mut World) {
  w.register::<components::debug::DebugRect>();
}
