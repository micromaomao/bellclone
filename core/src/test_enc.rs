use crate::ec::components::EntityId;

use super::enc::encode_entity_id;
use super::dec::parse_entity_id;
use uuid::Uuid;

#[test]
fn test_uuid_enc() {
  let t = Uuid::new_v4();
  assert_eq!(parse_entity_id(&encode_entity_id(EntityId(t))), EntityId(t));
}
