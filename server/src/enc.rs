use game_core::{ec::components::{
  physics::Velocity, player::PlayerComponent, transform::WorldSpaceTransform, EntityId,
}, enc::encode_entity_id};
use protocol::{flatbuffers::{FlatBufferBuilder, WIPOffset}, servermsg_generated::{PlayerDeleteBuilder, PlayerUpdate, PlayerUpdateBuilder, ServerMessage, ServerMessageBuilder, ServerMessageInner}};


pub fn encode_player_update<'a>(
  fbb: &mut FlatBufferBuilder<'a>,
  player_c: &PlayerComponent,
  entity_id: &EntityId,
  tr: &WorldSpaceTransform,
  vel: &Velocity,
) -> WIPOffset<ServerMessage<'a>> {
  let mut b = PlayerUpdateBuilder::new(fbb);
  let id = game_core::enc::encode_entity_id(*entity_id);
  b.add_id(&id);
  let score = game_core::enc::encode_score(player_c.score);
  b.add_score(&score);
  let pos = tr.position();
  let pos = protocol::base_generated::Vec2::new(pos.x, pos.y);
  b.add_pos(&pos);
  let vel = vel.0;
  let vel = protocol::base_generated::Vec2::new(vel.x, vel.y);
  b.add_vel(&vel);
  let msg = b.finish();
  to_message(fbb, msg, ServerMessageInner::PlayerUpdate)
}

pub fn encode_player_delete<'a>(
  fbb: &mut FlatBufferBuilder<'a>,
  entity_id: EntityId
) -> WIPOffset<ServerMessage<'a>> {
  let mut b = PlayerDeleteBuilder::new(fbb);
  let uuid = encode_entity_id(entity_id);
  b.add_id(&uuid);
  let msg = b.finish();
  to_message(fbb, msg, ServerMessageInner::PlayerDelete)
}

pub fn to_message<'a, Msg: 'a>(
  fbb: &mut FlatBufferBuilder<'a>,
  msg: WIPOffset<Msg>,
  ty: ServerMessageInner,
) -> WIPOffset<ServerMessage<'a>> {
  let mut b = ServerMessageBuilder::new(fbb);
  b.add_msg_type(ty);
  b.add_msg(msg.as_union_value());
  b.finish()
}
