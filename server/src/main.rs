use std::{
  future::Future,
  mem::{self},
  net::SocketAddr,
  ops::{Deref, DerefMut},
  println,
  sync::Arc,
  thread,
  time::{Duration, Instant},
};

use clap::Arg;
use enc::{encode_bell, encode_player_delete};
use futures::{FutureExt, SinkExt, StreamExt};
use game_core::{dec::parse_score, ec::{DeltaTime, components::{
      bell::BellComponent,
      physics::Velocity,
      player::{build_player, PlayerComponent},
      transform::WorldSpaceTransform,
      EntityId,
    }, register_common_components, register_common_systems, systems::create_bell::CreateBellSystemControl}, enc::encode_entity_id};
use glam::{f32::*, Vec3Swizzles};
use protocol::{clientmsg_generated::{root_as_client_message, ClientMessage, ClientMessageInner}, flatbuffers::FlatBufferBuilder, servermsg_generated::{BellsBuilder, ServerMessageInner, YourIDIs, YourIDIsBuilder}};
use specs::{Builder, Dispatcher, DispatcherBuilder, Entity, Join, World, WorldExt};
use std::error::Error;
use thread::sleep;
use tokio::{
  net::{TcpListener, TcpStream},
  sync::{broadcast, Mutex},
  time::interval,
};
use tokio_stream::wrappers::BroadcastStream;
use tokio_tungstenite::tungstenite::Message;

use crate::enc::to_message;

mod ec;
mod enc;

struct ServerContext {
  broadcast_server_message: broadcast::Sender<Arc<[u8]>>,
  ecworld: Mutex<World>,
}

/// Because dispatcher: !Send
struct MainThreadContext {
  dispatch: Dispatcher<'static, 'static>,
  last_update: Instant,
}

impl ServerContext {
  pub fn new() -> (ServerContext, MainThreadContext) {
    let mut w = World::new();
    register_common_components(&mut w);
    w.insert(DeltaTime::default());
    let mut ctl = CreateBellSystemControl::default();
    ctl.enabled = true;
    w.insert(ctl);
    let server_ctx = ServerContext {
      broadcast_server_message: broadcast::channel(100).0,
      ecworld: Mutex::new(w),
    };
    let mut dispatch = DispatcherBuilder::new();
    register_common_systems(&mut dispatch);
    dispatch.add(
      game_core::ec::systems::player::PlayerLimitSystem,
      "player_limit_system",
      &[],
    );
    dispatch.add(
      ec::ServerBellsSystem,
      "server_bells_system",
      &[]
    );
    let mt_ctx = MainThreadContext {
      dispatch: dispatch.build(),
      last_update: Instant::now(),
    };
    (server_ctx, mt_ctx)
  }

  pub fn subscribe_broadcast(&self) -> broadcast::Receiver<Arc<[u8]>> {
    self.broadcast_server_message.subscribe()
  }

  pub fn broadcast(&self, data: Vec<u8>) {
    let _ = self.broadcast_server_message.send(data.into());
  }

  pub fn borrow_world(
    &self,
  ) -> impl Future<Output = impl Deref<Target = World> + DerefMut + Send + Sync + '_> + Send + Sync + '_
  {
    self.ecworld.lock()
  }

  pub fn update<'a, 'b>(&self, rt: &tokio::runtime::Runtime, mt_ctx: &mut MainThreadContext) {
    let mut w = rt.block_on(self.borrow_world());
    let now = Instant::now();
    let dt = now.duration_since(mt_ctx.last_update);
    mt_ctx.last_update = now;
    let dt = DeltaTime::from_secs_f32(dt.as_secs_f32());
    *w.write_resource::<DeltaTime>() = dt;
    mt_ctx.dispatch.dispatch(&*w);
    let mut max_y = 0f32;
    {
      let players = w.read_storage::<PlayerComponent>();
      let poses = w.read_storage::<WorldSpaceTransform>();
      for (player, pos) in (&players, &poses).join() {
        let thisy = pos.position().y;
        if max_y < thisy {
          max_y = thisy;
        }
      }
    }
    {
      let mut need_to_broadcast_bells = &mut w.write_resource::<CreateBellSystemControl>().last_round_gen;
      let poses = w.read_storage::<WorldSpaceTransform>();
      let bells = w.read_storage::<BellComponent>();
      let vel = w.read_storage::<Velocity>();
      let mut fbb = FlatBufferBuilder::new();
      let mut buf = Vec::new();
      for &e in need_to_broadcast_bells.iter() {
        if let Some(&BellComponent { size }) = bells.get(e) {
          if let Some(wt) = poses.get(e) {
            let b = encode_bell(&mut fbb, bells.get(e).unwrap().size, &wt.position().xy(), &vel.get(e).unwrap().0);
            buf.push(b);
          }
        }
      }
      let v = fbb.create_vector(&buf);
      let mut bells_msg = BellsBuilder::new(&mut fbb);
      bells_msg.add_bells(v);
      let bells_msg = bells_msg.finish();
      let msg = to_message(&mut fbb, bells_msg, ServerMessageInner::Bells);
      fbb.finish(msg, None);
      self.broadcast(fbb.finished_data().to_vec());
      need_to_broadcast_bells.clear();
    }
    w.maintain();
  }
}

fn main() {
  let args = clap::App::new("server")
    .arg(
      Arg::with_name("listen")
        .index(1)
        .default_value("127.0.0.1:5000"),
    )
    .get_matches();
  let async_rt = tokio::runtime::Runtime::new().unwrap();
  {
    let (server_ctx, mut mt_ctx) = ServerContext::new();
    let server_ctx_static: &'static ServerContext = unsafe { mem::transmute(&server_ctx) };
    let mut joins = Vec::new();
    joins.push(async_rt.spawn(listen_ws(
      args.value_of("listen").unwrap().to_owned(),
      server_ctx_static,
    )));
    const MIN_DELAY: Duration = Duration::from_millis(20);
    loop {
      let start = mt_ctx.last_update;
      server_ctx.update(&async_rt, &mut mt_ctx);
      let dur = mt_ctx.last_update.duration_since(start);
      if dur < MIN_DELAY {
        sleep(MIN_DELAY - dur);
      }
    }
    for j in joins.into_iter() {
      j.abort();
    }
  }
}

async fn listen_ws(
  bind_addr: String,
  server_ctx: &'static ServerContext,
) -> Result<(), Box<dyn Error + Send + Sync>> {
  let socket = TcpListener::bind(bind_addr).await.unwrap();
  loop {
    let peer = socket.accept().await?;
    tokio::spawn(accept_ws(peer, server_ctx));
  }
}

async fn accept_ws(
  (tcpstream, addr): (TcpStream, SocketAddr),
  server_ctx: &'static ServerContext,
) -> Result<(), Box<dyn Error + Send + Sync>> {
  println!("Accepting connection from {}", addr);
  let mut ws = tokio_tungstenite::accept_async(tcpstream).await?;
  let mut fbb = protocol::flatbuffers::FlatBufferBuilder::new();
  let player_uuid;
  let player_ent;

  struct PlayerUpdateBody<'a, 'b, 'c, 'd>(
    &'a PlayerComponent,
    &'b EntityId,
    &'c WorldSpaceTransform,
    &'d Velocity,
  );

  macro_rules! get_current_player_state {
    ($w:expr) => {
      PlayerUpdateBody(
        $w.read_storage::<PlayerComponent>()
          .get(player_ent)
          .unwrap(),
        &player_uuid,
        $w.read_storage::<WorldSpaceTransform>()
          .get(player_ent)
          .unwrap(),
        $w.read_storage::<Velocity>().get(player_ent).unwrap(),
      )
    };
  }
  fn send_player_update(
    server_ctx: &'static ServerContext,
    fbb: &mut FlatBufferBuilder,
    PlayerUpdateBody(player_c, entity_id, tr, vel): PlayerUpdateBody<'_, '_, '_, '_>,
  ) {
    fbb.reset();
    let msg = enc::encode_player_update(fbb, player_c, entity_id, tr, vel);
    fbb.finish(msg, None);
    server_ctx.broadcast(fbb.finished_data().to_vec());
  }
  async fn process_client_msg<'a>(
    server_ctx: &'static ServerContext,
    msg: ClientMessage<'a>,
    player_ent: Entity,
  ) -> Result<(), ()> {
    match msg.msg_type() {
      ClientMessageInner::PlayerPosition => {
        let msg = msg.msg_as_player_position().ok_or(())?;
        let pos = msg.pos().ok_or(())?;
        let new_wt = WorldSpaceTransform::from_pos(Vec3::new(pos.x(), pos.y(), 0f32));
        let vel = msg.vel().ok_or(())?;
        let new_vel = Velocity(Vec2::new(vel.x(), vel.y()));
        let w = server_ctx.borrow_world().await;
        w.write_storage::<WorldSpaceTransform>()
          .insert(player_ent, new_wt);
        w.write_storage::<Velocity>().insert(player_ent, new_vel);
        Ok(())
      }
      ClientMessageInner::PlayerScore => {
        let msg = msg.msg_as_player_score().ok_or(())?;
        let score = msg.new_score().ok_or(())?;
        let score = parse_score(&score);
        let w = server_ctx.borrow_world().await;
        w.write_storage::<PlayerComponent>()
          .get_mut(player_ent)
          .unwrap()
          .score = score;
        Ok(())
      }
      _ => Err(()),
    }
  }

  {
    let mut w = server_ctx.borrow_world().await;
    player_ent = build_player(&mut *w).build();
    player_uuid = *w.read_storage::<EntityId>().get(player_ent).unwrap();
    send_player_update(server_ctx, &mut fbb, get_current_player_state!(w));

    fbb.reset();
    let bells = w.read_storage::<BellComponent>();
    let poses = w.read_storage::<WorldSpaceTransform>();
    let vel = w.read_storage::<Velocity>();
    let mut buf = Vec::new();
    for (b, wt, vel) in (&bells, &poses, &vel).join() {
      let msg = encode_bell(&mut fbb, b.size, &wt.position().xy(), &vel.0);
      buf.push(msg);
    }
    let v = fbb.create_vector(&buf);
    let mut bells_msg = BellsBuilder::new(&mut fbb);
    bells_msg.add_bells(v);
    let bells_msg = bells_msg.finish();
    let msg = to_message(&mut fbb, bells_msg, ServerMessageInner::Bells);
    fbb.finish(msg, None);
    ws.feed(Message::Binary(fbb.finished_data().to_vec())).await;
    fbb.reset();
  }

  {
    let mut idmsg = YourIDIsBuilder::new(&mut fbb);
    idmsg.add_id(&encode_entity_id(player_uuid));
    let idmsg = idmsg.finish();
    let msg = enc::to_message(&mut fbb, idmsg, ServerMessageInner::YourIDIs);
    fbb.finish(msg, None);
    ws.feed(Message::Binary(fbb.finished_data().to_vec())).await;
    fbb.reset();
  }

  ws.flush().await;

  let mut delay_fut = interval(Duration::from_millis(50));
  let broadcast_sub = server_ctx.subscribe_broadcast();
  let mut broadcast_sub = BroadcastStream::new(broadcast_sub);
  let mut player_changed = false;
  loop {
    fbb.reset();
    futures::select! {
      data = broadcast_sub.next().fuse() => {
        if let Some(Ok(data)) = data {
          ws.send(Message::Binary(data.to_vec())).await;
        }
      },
      _ = delay_fut.tick().fuse() => {{
        if player_changed {
          let w = server_ctx.borrow_world().await;
          send_player_update(server_ctx, &mut fbb, get_current_player_state!(w));
          player_changed = false;
        }
      }},
      data = ws.next().fuse() => {
        match data {
          Some(Ok(Message::Binary(data))) => {
            match root_as_client_message(&data) {
              Ok(msg) => {
                if process_client_msg(server_ctx, msg, player_ent).await.is_ok() {
                  player_changed = true;
                }
              },
              Err(_) => {}
            }
          },
          Some(Err(_)) | None => {
            break;
          },
          _ => {}
        }
      }
    }
    fbb.reset();
  }
  fbb.reset();
  let msg = encode_player_delete(&mut fbb, player_uuid);
  fbb.finish(msg, None);
  server_ctx.broadcast(fbb.finished_data().to_vec());
  server_ctx.borrow_world().await.delete_entity(player_ent);
  Ok(())
}
