use super::ws_messages::{ClientActorMessage, Connect, Disconnect, WsMessage};
use crate::{components::game::database, utils::enums::ServerMessage};
use crate::models::game::Game;
use actix::{
  prelude::{Actor, Context, Handler, Recipient},
  AsyncContext, ContextFutureSpawner, WrapFuture,
};
use async_trait::async_trait;
use futures::{Future, TryStreamExt};
use mongodb::Database;
use serde_json::json;
use std::{
  collections::{HashMap, HashSet},
  sync::{Arc, Mutex},
};
use uuid::Uuid;

type Socket = Recipient<WsMessage>;

// GameServer actor who keeps track of all the sessions and game rooms (each game room has up to 4 sessions)
pub struct GameServer {
  db: Arc<Mutex<Database>>,
  // player_id => Addres to send messages
  sessions: HashMap<String, Recipient<WsMessage>>,
  // room_id / game_id => player_id 
  rooms: HashMap<String, HashSet<String>>,
}

impl GameServer {
  pub fn new(db: Arc<Mutex<Database>>) -> Self {
    GameServer {
      db,
      sessions: HashMap::new(),
      rooms: HashMap::new(),
    }
  }
}

// helper fnc to send a message to a session actor with given id, session actor then sends this to the client
fn send_message(message: &str, sessions: HashMap<String, Recipient<WsMessage>>, id_to: &String) {
  if let Some(session) = sessions.get(id_to) {
    session.do_send(WsMessage(message.to_owned())).unwrap();
  } else {
    println!("attempting to send message but couldn't find session with given id.");
  }
}

// Make the game server an actor so it can recieve and send messages to sessions
impl Actor for GameServer {
  type Context = Context<Self>;
}

// Connect a session to the GameServer
impl Handler<Connect> for GameServer {
  type Result = ();

  fn handle(&mut self, msg: Connect, _: &mut Context<Self>) {
    println!("Connect handle");
    self.sessions.insert(msg.player_id.clone(), msg.address);
    self.rooms
      .entry(msg.room_id.clone())
      .or_insert_with(HashSet::new)
      .insert(msg.player_id.clone());

    if let Some(sessions) = self.rooms.get(&msg.room_id) {
      let count = sessions.iter().count();
      for session in sessions {
        let server_msg = ServerMessage::PlayerJoined(count);
        let json = serde_json::to_string(&server_msg).unwrap();
        let message = json.as_str();
        send_message(message, self.sessions.clone(), session);
      }
    }
  }
}

// Handler for session message to disconnect from the GameServer
impl Handler<Disconnect> for GameServer {
  type Result = ();

  fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
    println!("Someone left the game");

    let mut rooms: Vec<String> = Vec::new();

    if self.sessions.remove(&msg.player_id).is_some() {
      for (game_id, sessions) in &mut self.rooms {
        if sessions.remove(&msg.player_id) {
          rooms.push(game_id.to_owned());
        }
      }
    }

    for room in rooms {
      if let Some(sessions) = self.rooms.get(&room) {
        for session in sessions {
          send_message("Someone disconnected", self.sessions.clone(), session);
        }
      }
    }
  }
}

#[async_trait]
impl Handler<ClientActorMessage> for GameServer {
  type Result = ();

  fn handle(&mut self, msg: ClientActorMessage, ctx: &mut Context<Self>) {
    //   let sessions = self.sessions.clone();
    //   let db = self.db.clone();
    //   tokio::spawn(async move {
    //     let mut cursor = db.lock().unwrap().collection("games").find(None, None).await.unwrap();
    //     let mut games: Vec<Game> = Vec::new();
    //     while let Some(game) = cursor.try_next().await.unwrap() {
    //       games.push(game);
    //     }
    //     let json_string = serde_json::to_string(&games).unwrap();
    //     send_message(&json_string, sessions, &msg.id);
    //   });

    //   let fut = actix::fut::wrap_future::<_, Self>(future);
    //   ctx.spawn(fut);
    // }

    // fn handle(&mut self, msg: ClientActorMessage, ctx: &mut Context<Self>) {
    //   let sessions = self.sessions.clone();
    //   let db = self.db.clone();

    //   actix_web::rt::spawn(async move {
    //     let player_id = msg.id;
    //     let id = database::create_game(db, player_id).await.unwrap();
    //     let json_string = serde_json::to_string(&id).unwrap();
    //     send_message(&json_string, sessions, &msg.id);
    //   });

    //   let fut = actix::fut::wrap_future::<_, Self>(future);
    //   ctx.spawn(fut);
  }
}
