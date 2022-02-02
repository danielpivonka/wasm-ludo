use actix::{
  fut, Actor, ActorContext, ActorFutureExt, Addr, AsyncContext, ContextFutureSpawner, Handler,
  Running, StreamHandler, WrapFuture,
};
use actix_web_actors::ws;
use std::time::{Duration, Instant};

use crate::components::game_server::actor::GameServer;
use crate::models::actor_messages::{ClientActorMessage, Connect, Disconnect, WsMessage};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(10);
const TIMEOUT: Duration = Duration::from_secs(15);

// Game session actor (for each connected client)
// Sends messages to the GameServer actor who coordinates all connected sessions
pub struct GameSession {
  id: String,
  room: String,
  game_server: Addr<GameServer>,
  heartbeat: Instant,
}

impl GameSession {
  pub fn new(id: String, room: String, game_server: Addr<GameServer>) -> Self {
    println!("created game session");
    GameSession {
      id,
      room,
      heartbeat: Instant::now(),
      game_server,
    }
  }

  // start the heartbeat (ping the client and expect a pong message sent back)
  // if we get a pong message back, reset the last heartbeat -> can be seen later in this file
  // if the duration since the last pong message is greater than the timeout, we disconnect the session due to inactivity
  // fn heartbeat(&self, ctx: &mut ws::WebsocketContext<Self>) {
  //   ctx.run_interval(HEARTBEAT_INTERVAL, |actor, ctx| {
  //     if Instant::now().duration_since(actor.heartbeat) > TIMEOUT {
  //       println!("Disconnecting due to timeout");
  //       actor.game_server.do_send(Disconnect {
  //         player_id: actor.id.clone(),
  //         room_id: actor.room.clone(),
  //       });
  //       ctx.stop();
  //       return;
  //     };
  //     ctx.ping(b"PING");
  //   });
  // }
}

// implementing lifecycle methods for a session
impl Actor for GameSession {
  type Context = ws::WebsocketContext<Self>;

  fn started(&mut self, ctx: &mut Self::Context) {
    // self.heartbeat(ctx);

    println!("session started");

    let address = ctx.address();
    self
      .game_server
      .send(Connect {
        address: address.recipient(),
        player_id: self.id.clone(),
        room_id: self.room.clone(),
      })
      .into_actor(self)
      .then(|res, _, ctx| {
        if res.is_err() {
          ctx.stop();
        }
        fut::ready(())
      })
      .wait(ctx);

    ctx.text("connected");
  }

  fn stopping(&mut self, _: &mut Self::Context) -> Running {
    println!("stoppping");
    self.game_server.do_send(Disconnect {
      room_id: self.room.clone(),
      player_id: self.id.clone(),
    });
    Running::Stop
  }
}

/// Handler for messages coming from the client
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for GameSession {
  fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
    match msg {
      Ok(ws::Message::Ping(msg)) => {
        self.heartbeat = Instant::now();
        ctx.pong(&msg);
      }
      // we recieved a pong message, client is still active so we can reset the heartbeat
      Ok(ws::Message::Pong(_msg)) => {
        self.heartbeat = Instant::now();
      }
      Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
      Ok(ws::Message::Close(reason)) => {
        ctx.close(reason);
        ctx.stop();
      }
      Ok(ws::Message::Continuation(_)) => {
        // message is too large so its sent in continuation
        // we are not handling large data
        ctx.stop();
      }
      Ok(ws::Message::Nop) => {}
      Ok(ws::Message::Text(s)) => self.game_server.do_send(ClientActorMessage {
        player_id: self.id.clone(),
        content: s.to_string(),
        room_id: self.room.clone(),
      }),
      Err(e) => panic!("{}", e),
    }
  }
}

// Sending a message back to the client from the session actor
impl Handler<WsMessage> for GameSession {
  type Result = ();

  fn handle(&mut self, msg: WsMessage, ctx: &mut Self::Context) {
    // sending text to the client
    ctx.text(msg.0);
  }
}
