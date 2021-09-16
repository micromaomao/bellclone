use glam::Mat4;

use crate::ec::components::EntityId;

pub fn parse_entity_id(proto_id: &protocol::base_generated::Uuid) -> EntityId {
  EntityId(uuid::Uuid::from_u128(
    proto_id.lower() as u128 | ((proto_id.upper() as u128) << 64u128),
  ))
}

pub fn parse_score(score: &protocol::base_generated::Score) -> u128 {
  score.lower() as u128 | ((score.upper() as u128) << 64u128)
}

pub fn parse_mat4(m: &protocol::base_generated::Mat4) -> Mat4 {
  Mat4::from_cols_array(&[m.a1(), m.a2(), m.a3(), m.a4(), m.b1(), m.b2(), m.b3(), m.b4(), m.c1(), m.c2(), m.c3(), m.c4(), m.d1(), m.d2(), m.d3(), m.d4()])
}
