use std::error::Error;

use golem::Texture;
use image::ImageDecoder;
use image::{png::PngDecoder, ColorType};

pub struct LoadedTexture {
  pub gl_texture: Texture,
  pub width: u32,
  pub height: u32,
}

impl LoadedTexture {
  pub fn from_png_data(glctx: &golem::Context, data: &[u8]) -> Result<Self, Box<dyn Error>> {
    let decoded = PngDecoder::new(data)?;
    if decoded.color_type() != ColorType::Rgba8 {
      panic!("Invalid pixel format - only RGBA8 supported.");
    }
    let mut buf = vec![0u8; decoded.total_bytes() as usize];
    let (width, height) = decoded.dimensions();
    decoded.read_image(&mut buf[..])?;
    let mut tex = Texture::new(&glctx).map_err(|e| e.to_string())?;
    tex.set_image(Some(&buf[..]), width, height, golem::ColorFormat::RGBA);
    tex
      .set_minification(golem::TextureFilter::LinearMipmapLinear)
      .map_err(|e| e.to_string())?;
    Ok(LoadedTexture {
      gl_texture: tex,
      width,
      height,
    })
  }
}

macro_rules! define_images {
  ($($name:ident),+) => {
    pub struct Images {
      $(pub $name: LoadedTexture),+
    }

    impl Images {
      pub fn load(glctx: &golem::Context) -> Result<Self, Box<dyn Error>> {
        Ok(Images {
          $($name: LoadedTexture::from_png_data(glctx, include_bytes!(concat!("./images/", stringify!($name), ".png")))?),+
        })
      }
    }
  };
}

define_images!(crab, gopher, star, numbers);
