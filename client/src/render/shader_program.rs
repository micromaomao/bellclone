use golem::Dimension::*;
use golem::{Attribute, AttributeType, ShaderDescription, ShaderProgram, Uniform, UniformType};

pub struct Shaders {
  pub debug_rect: ShaderProgram,
  pub image: ShaderProgram,
}
fn fix_shader_source(source: &'static str) -> &'static str {
  if let Some(sep_comment_start) = source.find("////\n") {
    source.split_at(sep_comment_start).1
  } else {
    source
  }
}

macro_rules! src {
  ($path:literal) => {
    fix_shader_source(include_str!($path))
  };
}

impl Shaders {
  pub fn load(glctx: &golem::Context) -> Result<Self, golem::GolemError> {
    Ok(Shaders {
      debug_rect: ShaderProgram::new(
        glctx,
        ShaderDescription {
          vertex_input: &[Attribute::new("aVertexPosition", AttributeType::Vector(D2))],
          fragment_input: &[],
          uniforms: &[
            Uniform::new("uViewMat", UniformType::Matrix(D4)),
            Uniform::new("uObjectTransform", UniformType::Matrix(D4)),
            Uniform::new("uSize", UniformType::Vector(golem::NumberType::Float, D2)),
          ],
          vertex_shader: src!("./shaders/debug/rect.vert.glsl"),
          fragment_shader: src!("./shaders/debug/rect.frag.glsl"),
        },
      )?,
      image: ShaderProgram::new(
        glctx,
        ShaderDescription {
          vertex_input: &[Attribute::new("aVertexPosition", AttributeType::Vector(D2))],
          fragment_input: &[Attribute::new("oTexCord", AttributeType::Vector(D2))],
          uniforms: &[
            Uniform::new("uViewMat", UniformType::Matrix(D4)),
            Uniform::new("uObjectTransform", UniformType::Matrix(D4)),
            Uniform::new("uSize", UniformType::Vector(golem::NumberType::Float, D2)),
            Uniform::new("tex ", UniformType::Sampler2D),
            Uniform::new("alpha", UniformType::Scalar(golem::NumberType::Float)),
          ],
          vertex_shader: src!("./shaders/image.vert.glsl"),
          fragment_shader: src!("./shaders/image.frag.glsl"),
        },
      )?,
    })
  }
}
