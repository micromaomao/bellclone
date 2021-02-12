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
use futures::{FutureExt, Sink, SinkExt, Stream, StreamExt};
use game_core::ec::{
  components::{
    physics::Velocity,
    player::{build_player, PlayerComponent},
    transform::WorldSpaceTransform,
    EntityId,
  },
  register_common_components, register_common_systems, DeltaTime,
};
use protocol::servermsg_generated::{
  PlayerUpdateBuilder, ServerMessage, ServerMessageBuilder, ServerMessageInner,
  ServerMessageInnerUnionTableOffset,
};
use specs::{Builder, Dispatcher, DispatcherBuilder, World, WorldExt};
use std::error::Error;
use thread::sleep;
use tokio::{
  net::{TcpListener, TcpStream},
  sync::{broadcast, Mutex},
  time::interval,
};
use tokio_stream::wrappers::BroadcastStream;
use tokio_tungstenite::tungstenite::Message;

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
    let server_ctx = ServerContext {
      broadcast_server_message: broadcast::channel(100).0,
      ecworld: Mutex::new(w),
    };
    let mut dispatch = DispatcherBuilder::new();
    register_common_systems(&mut dispatch);
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
    const MIN_DELAY: Duration = Duration::from_millis(8); // ~120fps
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
  {
    let mut w = server_ctx.borrow_world().await;
    player_ent = build_player(&mut *w).build();
    player_uuid = *w.read_storage::<EntityId>().get(player_ent).unwrap();
    fbb.reset();
    let off = enc::encode_player_update(
      &mut fbb,
      w.read_storage::<PlayerComponent>().get(player_ent).unwrap(),
      &player_uuid,
      w.read_storage::<WorldSpaceTransform>()
        .get(player_ent)
        .unwrap(),
      w.read_storage::<Velocity>().get(player_ent).unwrap(),
    );
    let msg = enc::to_message(&mut fbb, off, ServerMessageInner::PlayerUpdate);
    fbb.finish(msg, None);
  }
  server_ctx.broadcast(fbb.finished_data().to_vec());
  let mut delay_fut = interval(Duration::from_millis(100));
  let broadcast_sub = server_ctx.subscribe_broadcast();
  let mut broadcast_sub = BroadcastStream::new(broadcast_sub);
  loop {
    fbb.reset();
    futures::select! {
      data = broadcast_sub.next().fuse() => {
        if let Some(Ok(data)) = data {
          ws.send(Message::Binary(data.to_vec())).await;
        }
      },
      _ = delay_fut.tick().fuse() => {
        // todo
      },
      _data = ws.next().fuse() => {
        // todo
      }
    }
  }
  Ok(())
}
