use game_core::ec::components::transform::WorldSpaceTransform;
use glam::f32::*;

use crate::render::view::ViewportInfo;

#[derive(Debug, Clone, Copy, Default)]
pub struct PointerState {
  pub pixel_pos: Option<(u32, u32)>,
  pub world_space: Option<Vec2>,
  pub pressing: bool,
  pub clicked: bool,
}

impl PointerState {
  pub fn is_hovering_on(&self, obj: WorldSpaceTransform, width: f32, height: f32) -> bool {
    let center = obj.position();
    if (center.z - 0f32).abs() > 0.0001f32 {
      panic!("Expected z to be 0, got {}.", center.z);
    }
    if let Some(ws) = self.world_space {
      (center.x - ws.x).abs() < width / 2f32 && (center.y - ws.y).abs() < height / 2f32
    } else {
      false
    }
  }

  pub fn update_pos(&mut self, pixel_pos: (u32, u32)) {
    self.pixel_pos.replace(pixel_pos);
  }

  pub fn recalculate_raycast(&mut self, viewport: &ViewportInfo) {
    if let Some(pixel_pos) = self.pixel_pos {
      self
        .world_space
        .replace(viewport.raycast(pixel_pos.0, pixel_pos.1));
    } else {
      self.world_space = None;
    }
  }

  pub fn mousedown(&mut self) {
    self.pressing = true;
    self.clicked = true;
  }

  pub fn mouseup(&mut self) {
    self.pressing = false;
  }

  pub fn frame(&mut self) {
    self.clicked = false;
  }
}
