use game_core::{
  ec::components::{physics::Velocity, transform::WorldSpaceTransform},
  enc::encode_score,
};
use protocol::{
  clientmsg_generated::{
    ClientMessage, ClientMessageBuilder, ClientMessageInner, PlayerPositionBuilder,
    PlayerScoreBuilder,
  },
  flatbuffers::{FlatBufferBuilder, WIPOffset},
};

pub fn encode_player_position<'a>(
  fbb: &mut FlatBufferBuilder<'a>,
  tr: &WorldSpaceTransform,
  vel: &Velocity,
) -> WIPOffset<ClientMessage<'a>> {
  let mut b = PlayerPositionBuilder::new(fbb);
  let pos = tr.position();
  let pos = protocol::base_generated::Vec2::new(pos.x, pos.y);
  b.add_pos(&pos);
  let vel = vel.0;
  let vel = protocol::base_generated::Vec2::new(vel.x, vel.y);
  b.add_vel(&vel);
  let msg = b.finish();
  to_message(fbb, msg, ClientMessageInner::PlayerPosition)
}

pub fn encode_player_score<'a>(
  fbb: &mut FlatBufferBuilder<'a>,
  score: u128,
) -> WIPOffset<ClientMessage<'a>> {
  let mut b = PlayerScoreBuilder::new(fbb);
  let s = encode_score(score);
  b.add_new_score(&s);
  let msg = b.finish();
  to_message(fbb, msg, ClientMessageInner::PlayerScore)
}

pub fn to_message<'a, Msg: 'a>(
  fbb: &mut FlatBufferBuilder<'a>,
  msg: WIPOffset<Msg>,
  ty: ClientMessageInner,
) -> WIPOffset<ClientMessage<'a>> {
  let mut b = ClientMessageBuilder::new(fbb);
  b.add_msg_type(ty);
  b.add_msg(msg.as_union_value());
  b.finish()
}
