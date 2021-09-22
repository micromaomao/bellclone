use std::{
  num::NonZeroU32,
  ops::{Deref, DerefMut},
};

use game_core::ec::{
  register_common_components, register_common_systems,
  systems::{
    create_bell::CreateBellSystemControl, create_bird::CreateBirdSystemController,
    max_player_y::MaxPlayerY,
  },
  DeltaTime,
};
use golem::{ElementBuffer, GeometryMode, Surface, Texture, UniformValue, VertexBuffer};
use specs::{Dispatcher, DispatcherBuilder};
use specs::{World, WorldExt};
use user_input::PointerState;

use crate::{
  render::{DrawingCtx, GraphicsCtx, ViewportSize},
  webapi_utils::perf_now_f64,
};

pub mod components;
pub mod systems;
pub mod user_input;

#[derive(Debug, Clone, Copy, Default)]
pub struct BlurFlags {
  pub motion_blur_dy: f32,
}

pub const GAME_SPEED: f32 = 1f32;

pub struct EcCtx {
  pub world: World,
  game_dispatch: Dispatcher<'static, 'static>,
  render_dispatch: Dispatcher<'static, 'static>,
  last_time: f64,
  render_surface: Surface,
  postprocess_quad_buf: VertexBuffer,
  postprocess_quad_ele: ElementBuffer,
}

impl EcCtx {
  pub fn new(graphics: &GraphicsCtx) -> Self {
    let mut world = World::new();
    register_common_components(&mut world);
    register_client_components(&mut world);
    world.insert(BlurFlags::default());
    world.maintain();
    let glctx = &graphics.glctx;
    let mut buf = VertexBuffer::new(glctx).unwrap();
    buf.set_data(&[
      -1f32, -1f32, // bl
      1f32, -1f32, // br
      1f32, 1f32, // tr
      -1f32, 1f32, // tl
    ]);
    let mut ele = ElementBuffer::new(glctx).unwrap();
    ele.set_data(&[
      1, 2, 0, // /|
      3, // |/
    ]);
    let mut game_dispatch = build_game_dispatch();
    let mut render_dispatch = build_render_dispatch(graphics);
    game_dispatch.setup(&mut world);
    render_dispatch.setup(&mut world);
    EcCtx {
      world,
      game_dispatch,
      render_dispatch,
      last_time: perf_now_f64(),
      render_surface: Surface::new(&graphics.glctx, Texture::new(&graphics.glctx).unwrap())
        .unwrap(),
      postprocess_quad_buf: buf,
      postprocess_quad_ele: ele,
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

  pub fn resize(&mut self, gctx: &GraphicsCtx) {
    let ViewportSize {
      canvas_width,
      canvas_height,
      ..
    } = *gctx.viewport_size.borrow();
    self.render_surface.bind();
    let mut tex = self.render_surface.take_texture().unwrap();
    tex.set_image(None, canvas_width, canvas_height, golem::ColorFormat::RGB);
    self.render_surface.put_texture(tex);
    Surface::unbind(&gctx.glctx);
  }

  pub fn render(&mut self, dctx: DrawingCtx) {
    let glctx = dctx.glctx;
    let ViewportSize {
      canvas_width,
      canvas_height,
      ..
    } = dctx.viewport.size;
    self.render_surface.bind();
    glctx.set_viewport(0, 0, canvas_width, canvas_height);
    glctx.set_clear_color(
      (51f32 / 255f32).powf(2.2f32),
      0f32,
      (102f32 / 255f32).powf(2.2f32),
      1f32,
    );
    glctx.clear();
    self.world.insert(dctx);
    self.render_dispatch.dispatch(&self.world);
    self.world.remove::<DrawingCtx>();
    Surface::unbind(glctx);
    let tex = unsafe { self.render_surface.borrow_texture().unwrap() };
    let mut shaders = dctx.shaders.borrow_mut();
    let prog = &mut shaders.postprocess;
    prog.bind();
    tex.set_active(NonZeroU32::new(1).unwrap());
    prog.set_uniform("tex", UniformValue::Int(1)).unwrap();
    let blur_flags = self.world.read_resource::<BlurFlags>();
    let view_height = dctx.viewport.tr.y - dctx.viewport.bl.y;
    let dt = self.world.read_resource::<DeltaTime>().as_secs_f32();
    prog
      .set_uniform(
        "mb_dist",
        UniformValue::Float(blur_flags.motion_blur_dy * dt / view_height / 2f32),
      )
      .unwrap();
    drop(blur_flags);
    unsafe {
      prog
        .draw(
          &self.postprocess_quad_buf,
          &self.postprocess_quad_ele,
          0..4,
          GeometryMode::TriangleStrip,
        )
        .unwrap();
    }
    self.pointer_state_mut().frame();
  }

  pub fn pointer_state_mut<'a>(&'a mut self) -> impl Deref<Target = PointerState> + DerefMut + 'a {
    self.world.write_resource::<PointerState>()
  }
}

fn register_client_components(w: &mut World) {
  w.register::<components::debug::DebugRect>();
  w.register::<components::player::OurPlayer>();
  w.register::<components::player::WithScoreDisplay>();
  w.register::<components::bell::OurJumpableBell>();
  w.register::<components::collision_star::CollisionStar>();
  w.register::<components::DrawImage>();
  w.register::<components::draw_numbers::DrawNumbersComponent>();
  w.register::<components::effects::FadeOut>();
  w.register::<components::BackgroundMarker>();
  w.register::<components::background_stars::BackgroundStar>();
}

fn build_game_dispatch<'a, 'b>() -> Dispatcher<'a, 'b> {
  let mut game_dispatch = DispatcherBuilder::new();
  register_common_systems(&mut game_dispatch);
  game_dispatch.add(
    game_core::ec::systems::player::PlayerLimitSystem,
    "player_limit_system",
    &["velocity_system"],
  );
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
  game_dispatch.add(
    systems::players::ShowPlayerScoreSystem,
    "show_player_score_system",
    &["our_player_system"],
  );
  game_dispatch.add(systems::effects::FadeOutSystem, "fade_out_system", &[]);
  game_dispatch.add(
    systems::restart::RestartSystem::default(),
    "restart_system",
    &[],
  );
  game_dispatch.add(
    systems::bird::ClientBirdSystem,
    "client_bird_system",
    &["bird_system"],
  );
  game_dispatch.add(
    systems::tutorial_and_score::TutorialAndScoreSystem::default(),
    "tutorial_and_score_system",
    &["our_player_system"],
  );
  game_dispatch.build()
}

fn build_render_dispatch<'a, 'b>(graphics: &GraphicsCtx) -> Dispatcher<'a, 'b> {
  let mut render_dispatch = DispatcherBuilder::new();
  let glctx = &graphics.glctx;
  render_dispatch.add_thread_local(systems::background_stars::BackgroundStarSystem::default());
  render_dispatch.add_thread_local(systems::draw_debug::DrawDebug::new(glctx).unwrap());
  render_dispatch.add_thread_local(systems::draw_image::DrawImageSystem::new(glctx).unwrap());
  render_dispatch.add_thread_local(systems::draw_numbers::DrawNumbersSystem::new(glctx).unwrap());
  render_dispatch.build()
}
