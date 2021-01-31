use specs::{Component, VecStorage};
use glam::f32::*;

use crate::render::image_texture::LoadedTexture;

pub mod debug;
pub mod player;

pub struct DrawImage {
  pub texture: &'static LoadedTexture,
  pub size: Vec2,
}

impl Component for DrawImage {
  type Storage = VecStorage<Self>;
}
