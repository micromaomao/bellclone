use golem::{ElementBuffer, VertexBuffer};
use specs::{Component, DenseVecStorage};

use crate::{global, render::numbers};

pub enum Align {
  Left,
  Center,
}

pub struct DrawNumbersComponent {
  buf: VertexBuffer,
  draw_size: usize,
  alpha: f32,
  nb_digits: usize,
  current_number: u128,
  pub align: Align,
}

pub fn construct_elem_buf(glctx: &golem::Context) -> Result<ElementBuffer, golem::GolemError> {
  let mut ebuf = ElementBuffer::new(glctx)?;
  let mut ele_data = Vec::new();
  // log(2^128) ~= 39
  for i in 0..39u32 {
    let start = i * 4u32;
    ele_data.extend_from_slice(&[
      start + 0,
      start + 1,
      start + 2,
      start + 1,
      start + 2,
      start + 3,
    ]);
  }
  ebuf.set_data(&ele_data);
  Ok(ebuf)
}

// Return the number of vertices needed to draw, based on the element buffer constructed by construct_elem_buf.
fn populate_bufs(digits: &[u8], buf: &mut VertexBuffer) -> usize {
  // For each vertex: x y tex_x, tex_y
  // width is 1
  const CHAR_HEIGHT: f32 = 2f32;
  let mut data = Vec::with_capacity(digits.len() * 24);
  /*
    0   1      .   1
            +

    2   .      2   3
  */
  for (i, &dig) in digits.iter().enumerate() {
    let tex_cords = numbers::get_digit_glyph_texcord(dig);
    let i = i as f32;
    data.extend_from_slice(&[i, CHAR_HEIGHT, tex_cords.left, tex_cords.top]);
    data.extend_from_slice(&[i + 1f32, CHAR_HEIGHT, tex_cords.right, tex_cords.top]);
    data.extend_from_slice(&[i, 0f32, tex_cords.left, tex_cords.bottom]);
    data.extend_from_slice(&[i + 1f32, 0f32, tex_cords.right, tex_cords.bottom]);
  }
  buf.set_data(&data);
  digits.len() * 6
}

pub fn number_to_digits(mut number: u128) -> Vec<u8> {
  let mut digits = Vec::new();
  if number == 0 {
    digits.push(0u8);
    return digits;
  }
  while number > 0 {
    digits.push((number % 10) as u8);
    number /= 10u128;
  }
  digits.reverse();
  digits
}

#[test]
fn test_number_to_digits() {
  assert_eq!(number_to_digits(0), &[0u8]);
  assert_eq!(number_to_digits(1), &[1u8]);
  assert_eq!(number_to_digits(15), &[1u8, 5u8]);
  assert_eq!(number_to_digits(50), &[5u8, 0u8]);
  assert_eq!(
    number_to_digits(123456789),
    &[1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8]
  );
}

impl DrawNumbersComponent {
  pub fn new(alpha: f32, align: Align) -> Self {
    let mut buf = VertexBuffer::new(&global::get_ref().graphics.glctx).unwrap();
    let s = populate_bufs(&[0u8], &mut buf);
    Self {
      buf,
      draw_size: s,
      alpha,
      nb_digits: 1,
      current_number: 0u128,
      align,
    }
  }

  pub fn set_number(&mut self, nb: u128) {
    if self.current_number == nb {
      return;
    }
    let buf = &mut self.buf;
    let digits = number_to_digits(nb);
    let s = populate_bufs(&digits, buf);
    self.draw_size = s;
    self.nb_digits = digits.len();
    self.current_number = nb;
  }

  pub fn alpha(&self) -> f32 {
    self.alpha
  }

  pub fn set_alpha(&mut self, alpha: f32) {
    self.alpha = alpha;
  }

  pub(crate) fn buf(&self) -> &VertexBuffer {
    &self.buf
  }

  pub(crate) fn draw_size(&self) -> usize {
    self.draw_size
  }

  pub(crate) fn nb_digits(&self) -> usize {
    self.nb_digits
  }
}

impl Component for DrawNumbersComponent {
  type Storage = DenseVecStorage<Self>;
}
