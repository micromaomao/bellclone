use glam::Mat4;

use crate::ec::components::EntityId;

pub fn encode_entity_id(entity_id: EntityId) -> protocol::base_generated::Uuid {
  let uuid = entity_id.0.as_u128();
  protocol::base_generated::Uuid::new((uuid & 0xffffffffffffffffu128) as u64, (uuid >> 64u128) as u64)
}

pub fn encode_score(score: u128) -> protocol::base_generated::Score {
  protocol::base_generated::Score::new((score & 0xffffffffffffffffu128) as u64, (score >> 64u128) as u64)
}

pub fn encode_mat4(m: Mat4) -> protocol::base_generated::Mat4 {
  let [a1, a2, a3, a4, b1, b2, b3, b4, c1, c2, c3, c4, d1, d2, d3, d4] = m.to_cols_array();
  protocol::base_generated::Mat4::new(a1, a2, a3, a4, b1, b2, b3, b4, c1, c2, c3, c4, d1, d2, d3, d4)
}
