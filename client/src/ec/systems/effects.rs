use game_core::ec::DeltaTime;
use specs::{Entities, Join, Read, System, WriteStorage};

use crate::ec::components::{DrawImage, draw_numbers::DrawNumbersComponent, effects::FadeOut};

pub struct FadeOutSystem;

impl<'a> System<'a> for FadeOutSystem {
  type SystemData = (
    Entities<'a>,
    Read<'a, DeltaTime>,
    WriteStorage<'a, FadeOut>,
    WriteStorage<'a, DrawNumbersComponent>,
    WriteStorage<'a, DrawImage>,
  );

  fn run(&mut self, (ents, dt, mut fade_outs, mut dns, mut dis): Self::SystemData) {
    let dt = dt.as_secs_f32();
    for (entid, fo) in (&ents, &mut fade_outs).join() {
      fo.alive += dt;
      if fo.alive > fo.total_time {
        ents.delete(entid).unwrap();
      }
    }
    for (fo, dn) in (&mut fade_outs, &mut dns).join() {
      dn.set_alpha(1f32 - fo.alive / fo.total_time);
    }
    for (fo, di) in (&mut fade_outs, &mut dis).join() {
      di.alpha = 1f32 - fo.alive / fo.total_time;
    }
  }
}
