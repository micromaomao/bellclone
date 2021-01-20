use crate::render::GraphicsCtx;
use std::mem::MaybeUninit;

pub struct Context {
  pub graphics: GraphicsCtx,
}
pub static mut game_ctx: MaybeUninit<Context> = MaybeUninit::uninit();
pub unsafe fn init_ctx(ctx: Context) {
  game_ctx = MaybeUninit::new(ctx);
}
pub fn get_ref() -> &'static Context {
  unsafe { &*game_ctx.as_ptr() }
}
