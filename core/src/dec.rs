use crate::ec::components::EntityId;

pub fn parse_entity_id(proto_id: &protocol::base_generated::Uuid) -> EntityId {
  EntityId(uuid::Uuid::from_u128(
    proto_id.lower() as u128 | ((proto_id.upper() as u128) << 64u128),
  ))
}

pub fn parse_score(score: &protocol::base_generated::Score) -> u128 {
  score.lower() as u128 | ((score.upper() as u128) << 64u128)
}
