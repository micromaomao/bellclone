use crate::ec::components::EntityId;

pub fn encode_entity_id(entity_id: EntityId) -> protocol::base_generated::Uuid {
  let uuid = entity_id.0.as_u128();
  protocol::base_generated::Uuid::new((uuid & 0xffffffffu128) as u64, (uuid >> 64u128) as u64)
}

pub fn encode_score(score: u128) -> protocol::base_generated::Score {
  protocol::base_generated::Score::new((score & 0xffffffffu128) as u64, (score >> 64u128) as u64)
}
