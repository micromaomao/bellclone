pub mod ec;
pub mod gen;
pub mod enc;
pub mod dec;

pub const STAGE_MIN_X: f32 = -8f32;
pub const STAGE_MAX_X: f32 = 8f32;
pub const STAGE_WIDTH: f32 = STAGE_MAX_X - STAGE_MIN_X;
pub const STAGE_MIN_HEIGHT: f32 = 9f32;
