use std::ops::{Deref, DerefMut};

use game_core::ec::{register_common_components, register_common_systems, DeltaTime};
use specs::{Dispatcher, DispatcherBuilder};
use specs::{World, WorldExt};
use user_input::PointerState;

use crate::{global, render::{DrawingCtx, GraphicsCtx}, webapi_utils::perf_now_f64};

pub mod components;
pub mod systems;
pub mod user_input;

pub const GAME_SPEED: f32 = 1f32;

pub struct EcCtx {
  pub world: World,
  game_dispatch: Dispatcher<'static, 'static>,
  render_dispatch: Dispatcher<'static, 'static>,
  last_time: f64,
}

impl EcCtx {
  pub fn new(graphics: &GraphicsCtx) -> Self {
    let mut world = World::new();
    register_common_components(&mut world);
    register_client_components(&mut world);
    world.insert(DeltaTime::default());
    world.insert(PointerState::default());
    world.maintain();
    EcCtx {
      world,
      game_dispatch: build_game_dispatch(),
      render_dispatch: build_render_dispatch(graphics),
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
      *w_dt = DeltaTime::from_secs_f32(new_dt / 1000f32 * GAME_SPEED);
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
  w.register::<components::bell::OurJumpableBell>();
  w.register::<components::collision_star::CollisionStar>();
  w.register::<components::DrawImage>();
}

fn build_game_dispatch<'a, 'b>() -> Dispatcher<'a, 'b> {
  let mut game_dispatch = DispatcherBuilder::new();
  register_common_systems(&mut game_dispatch);
  game_dispatch.add(
    systems::our_player::OurPlayerSystem,
    "our_player_system",
    &[],
  );
  game_dispatch.add(
    systems::bell::BellSystem,
    "bell_system",
    &["our_player_system"],
  );
  game_dispatch.add(
    systems::collision_star::CollisionStarSystem,
    "collision_star_system",
    &["bell_system"],
  );
  game_dispatch.build()
}

fn build_render_dispatch<'a, 'b>(graphics: &GraphicsCtx) -> Dispatcher<'a, 'b> {
  let mut render_dispatch = DispatcherBuilder::new();
  let glctx = &graphics.glctx;
  render_dispatch.add_thread_local(systems::draw_debug::DrawDebug::new(glctx).unwrap());
  render_dispatch.add_thread_local(systems::draw_image::DrawImageSystem::new(glctx).unwrap());
  render_dispatch.build()
}
