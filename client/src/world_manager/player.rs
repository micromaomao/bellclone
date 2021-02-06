use crate::{
  ec::{
    components::{
      debug::DebugRect,
      player::{OurPlayer, OurPlayerState},
      DrawImage,
    },
    EcCtx,
  },
  global,
};
use game_core::ec::components::{player::build_player, transform::WorldSpaceTransform};
use glam::f32::*;
use specs::{Builder, Entity, WorldExt};

pub fn create_player_local(ec: &mut EcCtx) -> Entity {
  build_player(&mut ec.world)
    .with(OurPlayer {
      state: OurPlayerState::NotStarted,
    })
    .with(DrawImage {
      texture: &global::get_ref().graphics.images.crab,
      size: Vec2::new(1f32, 1f32),
      alpha: 1f32,
    })
    .build()
}

pub fn create_background(ec: &mut EcCtx) -> Entity {
  ec.world
    .create_entity()
    .with(WorldSpaceTransform::from_pos(Vec3::zero()))
    .with(DebugRect::default())
    .build()
}
