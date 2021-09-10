use crate::{
  ec::EcCtx, render::GraphicsCtx, websocket::SocketContext, world_manager::WorldManager,
};
use std::{cell::RefCell, mem::MaybeUninit};

pub struct Context {
  pub graphics: GraphicsCtx,
  pub ec: RefCell<EcCtx>,
  pub world_manager: RefCell<WorldManager>,
  pub socket_context: SocketContext,
}
pub static mut game_ctx: MaybeUninit<Context> = MaybeUninit::uninit();
pub static mut initialized: bool = false;
pub unsafe fn init_ctx(ctx: Context) {
  game_ctx = MaybeUninit::new(ctx);
  initialized = true;
}
pub fn get_ref() -> &'static Context {
  // SAFETY: we are in a single-threaded world.
  unsafe {
    if !initialized {
      panic!("init_ctx not called yet.");
    }
    &*game_ctx.as_ptr()
  }
}
