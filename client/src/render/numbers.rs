pub struct TexCords {
  pub top: f32,
  pub left: f32,
  pub right: f32,
  pub bottom: f32,
}

pub fn get_digit_glyph_texcord(digit: u8) -> TexCords {
  const W: f32 = 512f32;
  const H: f32 = 512f32;
  const CW: f32 = 80f32;
  const CH: f32 = 160f32;
  if (0u8..=4u8).contains(&digit) {
    TexCords {
      top: 0f32,
      left: digit as f32 * CW / W,
      right: (digit + 1) as f32 * CW / W,
      bottom: CH / H,
    }
  } else if (5u8..=9u8).contains(&digit) {
    let i = digit - 5;
    TexCords {
      top: CH / H,
      left: i as f32 * CW / W,
      right: (i + 1) as f32 * CW / W,
      bottom: 2f32 * CH / H
    }
  } else {
    panic!("Invalid digit");
  }
}
