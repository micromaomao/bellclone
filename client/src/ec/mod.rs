use std::ops::{Deref, DerefMut};

use game_core::ec::{register_common_components, register_common_systems, DeltaTime};
use specs::{Dispatcher, DispatcherBuilder};
use specs::{World, WorldExt};
use systems::our_player::OurPlayerSystem;
use user_input::PointerState;

use crate::{render::DrawingCtx, webapi_utils::perf_now_f64};

pub mod components;
pub mod systems;
pub mod user_input;

pub struct EcCtx {
  pub world: World,
  game_dispatch: Dispatcher<'static, 'static>,
  render_dispatch: Dispatcher<'static, 'static>,
  last_time: f64,
}

impl EcCtx {
  pub fn new() -> Self {
    let mut world = World::new();
    register_common_components(&mut world);
    register_client_components(&mut world);
    world.insert(DeltaTime::default());
    world.insert(PointerState::default());
    let mut game_dispatch = DispatcherBuilder::new();
    register_common_systems(&mut game_dispatch);
    game_dispatch.add(OurPlayerSystem, "our_player_system", &[]);
    let mut render_dispatch = DispatcherBuilder::new();
    render_dispatch.add_thread_local(systems::draw_debug::DrawDebug);
    world.maintain();
    EcCtx {
      world,
      game_dispatch: game_dispatch.build(),
      render_dispatch: render_dispatch.build(),
      last_time: perf_now_f64(),
    }
  }

  pub fn update(&mut self) {
    let now = perf_now_f64();
    let mut new_dt = (now - self.last_time) as f32;
    self.last_time = now;
    if new_dt <= 0f32 {
      new_dt = 0f32;
      // ???
    }
    {
      let mut w_dt = self.world.write_resource::<DeltaTime>();
      *w_dt = DeltaTime::from_secs_f32(new_dt / 1000f32);
    }

    self.game_dispatch.dispatch(&self.world);
    self.world.maintain();
  }

  pub fn render(&mut self, dctx: DrawingCtx) {
    self.world.insert(dctx);
    self.render_dispatch.dispatch(&self.world);
    self.world.remove::<DrawingCtx>();
    self.pointer_state_mut().frame();
  }

  pub fn pointer_state_mut<'a>(&'a mut self) -> impl Deref<Target = PointerState> + DerefMut + 'a {
    self.world.write_resource::<PointerState>()
  }
}

fn register_client_components(w: &mut World) {
  w.register::<components::debug::DebugRect>();
  w.register::<components::player::OurPlayer>();
}
