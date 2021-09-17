use std::{collections::HashSet, f32::consts::PI};

use game_core::ec::{DeltaTime, components::transform::WorldSpaceTransform};
use glam::{Mat4, Quat, Vec2, Vec3};
use rand::Rng;
use specs::{Entities, Join, Read, System, WriteStorage};

use crate::{
  ec::components::{background_stars::BackgroundStar, BackgroundMarker, DrawImage},
  render::DrawingCtx,
};

#[derive(Default)]
pub struct BackgroundStarSystem {
  pub generated_regions: HashSet<(i64, i64)>
}

const STAR_FLASH_SPEED: f32 = 1f32;

impl<'a> System<'a> for BackgroundStarSystem {
  type SystemData = (
    Entities<'a>,
    WriteStorage<'a, BackgroundStar>,
    WriteStorage<'a, BackgroundMarker>,
    WriteStorage<'a, DrawImage>,
    WriteStorage<'a, WorldSpaceTransform>,
    Option<Read<'a, DrawingCtx>>,
    Read<'a, DeltaTime>,
  );

  fn run(&mut self, (ents, mut starc, mut bgmc, mut drawc, mut wstc, dwctx, deltatime): Self::SystemData) {
    let dt = deltatime.as_secs_f32();
    if dwctx.is_none() {
      return;
    }
    let dwctx = dwctx.unwrap();
    if starc.is_empty() {
      self.generated_regions.clear();
    }
    let viewport = &dwctx.viewport;
    let mut x = viewport.bl.x.floor();
    let mut y = viewport.bl.y.floor();
    let mut xlow: i64 = x.round() as i64;
    let mut ylow: i64 = y.round() as i64;
    let mut xhi: Option<i64> = None;
    let mut yhi: Option<i64> = None;
    loop {
      if x > viewport.tr.x {
        x = viewport.bl.x.floor();
        y += 1f32;
      } else {
        x += 1f32;
      }
      if y > viewport.tr.y {
        break;
      }
      let pair = (x.round() as i64, y.round() as i64);
      if xhi.is_none() || xhi.unwrap() < pair.0 {
        xhi = Some(pair.0);
      }
      if yhi.is_none() || yhi.unwrap() < pair.1 {
        yhi = Some(pair.1);
      }
      if !self.generated_regions.contains(&pair) {
        gen_region(&ents, &mut starc, &mut bgmc, &mut drawc, &mut wstc, x, y);
        self.generated_regions.insert(pair);
      }
    }

    self.generated_regions.retain(|(x, y)| {
      (xlow..=xhi.unwrap_or(xlow)).contains(x) && (ylow..=yhi.unwrap_or(ylow)).contains(y)
    });

    for (ent, wst, star, draw) in (&ents, &wstc, &mut starc, &mut drawc).join() {
      let pos = wst.position();
      let pos_pair = (pos.x.floor() as i64, pos.y.floor() as i64);
      if !self.generated_regions.contains(&pos_pair) {
        ents.delete(ent);
        continue;
      }
      let mut current_alpha = draw.alpha;
      if star.current_alpha_increasing {
        current_alpha += STAR_FLASH_SPEED * dt;
        if current_alpha > star.alpha_range.end {
          current_alpha = star.alpha_range.end;
          star.current_alpha_increasing = false;
        }
      } else {
        current_alpha -= STAR_FLASH_SPEED * dt;
        if current_alpha < star.alpha_range.start {
          current_alpha = star.alpha_range.start;
          star.current_alpha_increasing = true;
        }
      }
      draw.alpha = current_alpha;
    }
  }
}

fn gen_region<'a>(
  ents: &Entities,
  mut starc: &mut WriteStorage<'a, BackgroundStar>,
  mut bgmc: &mut WriteStorage<'a, BackgroundMarker>,
  mut drawc: &mut WriteStorage<'a, DrawImage>,
  mut wstc: &mut WriteStorage<'a, WorldSpaceTransform>,
  x: f32,
  y: f32,
) {
  let mut rng = rand::thread_rng();
  const NB: i32 = 3;
  for _ in 0..NB {
    let ent = ents.create();
    wstc.insert(
      ent,
      WorldSpaceTransform(Mat4::from_rotation_translation(
        Quat::from_rotation_z(rng.gen_range(-PI..PI)),
        Vec3::new(
          rng.gen_range(x..x + 1f32),
          rng.gen_range(y..y + 1f32),
          rng.gen_range(-2f32..-0.5f32),
        ),
      )),
    );
    let alpha_range = rng.gen_range(0.1f32..0.7f32)..rng.gen_range(0.9f32..1f32);
    starc.insert(
      ent,
      BackgroundStar {
        alpha_range: alpha_range.clone(),
        current_alpha_increasing: rng.gen_bool(0.5f64),
      },
    );
    bgmc.insert(ent, BackgroundMarker);
    let size = rng.gen_range(0.02f32..0.07f32);
    drawc.insert(ent, DrawImage {
      texture: &crate::global::get_ref().graphics.images.star,
      alpha: rng.gen_range(alpha_range),
      size: Vec2::new(size, size)
    });
  }
}
