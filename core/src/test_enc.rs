use crate::dec::parse_mat4;
use crate::ec::components::EntityId;
use crate::enc::encode_mat4;

use super::enc::encode_entity_id;
use super::dec::parse_entity_id;
use glam::Mat4;
use uuid::Uuid;

#[test]
fn test_uuid_enc() {
  let t = Uuid::new_v4();
  assert_eq!(parse_entity_id(&encode_entity_id(EntityId(t))), EntityId(t));
}

#[test]
fn test_mat_enc() {
  use std::iter::Iterator;
  let m = Mat4::from_cols_slice(&(0..16).map(|x| x as f32).collect::<Vec<_>>());
  assert_eq!(parse_mat4(&encode_mat4(m)), m);
}
