use game_core::ec::components::{player::PlayerComponent, transform::WorldSpaceTransform};
use glam::{Mat4, Vec2, Vec3};
use specs::{Builder, Entities, Entity, Join, Read, ReadStorage, System, WorldExt, WriteStorage};

use crate::ec::components::{
  draw_numbers::{Align, DrawNumbersComponent},
  effects::FadeOut,
  player::OurPlayer,
  DrawImage,
};

#[derive(Default)]
pub struct TutorialAndScoreSystem {
  player_scored_first: bool,
  tutorial: Option<Entity>,
  last_non_zero_score: u128,
  score_display_ent: Option<Entity>,
  score_display_label: Option<Entity>,
}

impl<'a> System<'a> for TutorialAndScoreSystem {
  type SystemData = (
    Entities<'a>,
    WriteStorage<'a, DrawImage>,
    WriteStorage<'a, WorldSpaceTransform>,
    ReadStorage<'a, OurPlayer>,
    ReadStorage<'a, PlayerComponent>,
    WriteStorage<'a, FadeOut>,
    WriteStorage<'a, DrawNumbersComponent>,
  );

  fn run(
    &mut self,
    (ents, mut drawc, mut wstc, our_player, playerc, mut fadec, mut drawnbc): Self::SystemData,
  ) {
    if !self.player_scored_first
      && (self.tutorial.is_none() || !ents.is_alive(self.tutorial.unwrap()))
    {
      let tut_ent = ents.create();
      self.tutorial = Some(tut_ent);
      wstc.insert(
        tut_ent,
        WorldSpaceTransform::from_pos(Vec3::new(-4f32, 0f32, 0f32)),
      );
      drawc.insert(
        tut_ent,
        DrawImage {
          texture: &crate::global::get_ref().graphics.images.tutorial,
          alpha: 1f32,
          size: Vec2::new(6.5f32, 6.5f32),
        },
      );
    } else if self.player_scored_first && self.tutorial.is_some() {
      let _ = fadec.insert(self.tutorial.unwrap(), FadeOut::new(1f32));
      self.tutorial = None;
    }
    for (_, player) in (&our_player, &playerc).join() {
      if player.score > 0 {
        self.player_scored_first = true;
        self.last_non_zero_score = player.score;
        if let Some(ent) = self.score_display_ent.take() {
          fadec.insert(ent, FadeOut::new(1f32));
          fadec.insert(self.score_display_label.unwrap(), FadeOut::new(1f32));
          self.score_display_ent = None;
          self.score_display_label = None;
        }
      } else if self.player_scored_first {
        if self.score_display_ent.is_none() || !ents.is_alive(self.score_display_ent.unwrap()) {
          self.score_display_ent = Some(ents.create());
          let ent = self.score_display_ent.unwrap();
          let mut dn = DrawNumbersComponent::new(1f32, Align::Left);
          dn.set_number(self.last_non_zero_score);
          drawnbc.insert(ent, dn);
          wstc.insert(
            ent,
            WorldSpaceTransform::from_pos(Vec3::new(-7f32, -1.5f32, 0f32))
              .add(Mat4::from_scale(Vec3::new(0.3f32, 0.3f32, 0.3f32))),
          );
          self.score_display_label = Some(ents.create());
          let ent = self.score_display_label.unwrap();
          drawc.insert(ent, DrawImage {
            texture: &crate::global::get_ref().graphics.images.your_score,
            size: Vec2::new(1.5f32, 1.5f32),
            alpha: 1f32
          });
          wstc.insert(ent, WorldSpaceTransform::from_pos(Vec3::new(-6.4f32, -0.7f32, 0f32)));
        }
      }
    }
  }
}
