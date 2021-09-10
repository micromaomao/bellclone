use game_core::ec::components::{player::PlayerComponent, transform::WorldSpaceTransform};
use glam::f32::*;
use specs::{Entities, Entity, Join, Read, ReadStorage, System, WriteStorage};

use crate::ec::components::{
  draw_numbers::{Align, DrawNumbersComponent},
  player::WithScoreDisplay,
};

const SCORE_SCALE: f32 = 0.1f32;

pub struct ShowPlayerScoreSystem;

impl<'a> System<'a> for ShowPlayerScoreSystem {
  type SystemData = (
    Entities<'a>,
    ReadStorage<'a, PlayerComponent>,
    WriteStorage<'a, WithScoreDisplay>,
    WriteStorage<'a, DrawNumbersComponent>,
    WriteStorage<'a, WorldSpaceTransform>,
  );

  fn run(&mut self, (ents, players, mut with_score_displays, mut dns, mut trs): Self::SystemData) {
    for (entid, player) in (&ents, &players).join() {
      let dnid;
      if let Some(score_display) = with_score_displays.get_mut(entid) {
        dnid = score_display.0;
      } else {
        dnid = ents.create();
        dns.insert(dnid, DrawNumbersComponent::new(1f32, Align::Center)).unwrap();
        with_score_displays.insert(entid, WithScoreDisplay(dnid)).unwrap();
      }
      let dn = dns.get_mut(dnid).unwrap();
      dn.set_number(player.score);
      let player_tr = *trs.get(entid).unwrap();
      trs.insert(
        dnid,
        player_tr.add(Mat4::from_scale_rotation_translation(
          Vec3::new(SCORE_SCALE, SCORE_SCALE, 1f32),
          Quat::default(),
          Vec3::Y * 0.3f32,
        )),
      ).unwrap();
    }
  }
}
