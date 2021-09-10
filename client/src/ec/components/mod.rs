use glam::f32::*;
use specs::{Component, VecStorage};

use crate::render::image_texture::LoadedTexture;

pub mod bell;
pub mod collision_star;
pub mod debug;
pub mod draw_numbers;
pub mod effects;
pub mod player;

pub struct DrawImage {
  pub texture: &'static LoadedTexture,
  pub size: Vec2,
  pub alpha: f32,
}

impl Component for DrawImage {
  type Storage = VecStorage<Self>;
}
