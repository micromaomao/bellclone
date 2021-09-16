use glam::f32::*;
use specs::{Component, NullStorage, VecStorage};

use crate::render::image_texture::LoadedTexture;

pub mod bell;
pub mod collision_star;
pub mod debug;
pub mod draw_numbers;
pub mod effects;
pub mod player;
pub mod background_stars;

pub struct DrawImage {
  pub texture: &'static LoadedTexture,
  pub size: Vec2,
  pub alpha: f32,
}

impl Component for DrawImage {
  type Storage = VecStorage<Self>;
}

#[derive(Default, Clone, Copy)]
pub struct BackgroundMarker;

impl Component for BackgroundMarker {
  type Storage = NullStorage<BackgroundMarker>;
}
