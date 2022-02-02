use actix::prelude::{Actor, Context, Handler, Recipient};
use mongodb::Database;
use std::{
  collections::{HashMap, HashSet},
  sync::{Arc, Mutex},
};

use super::{services::start_game::start_game, utils::send_message_to_room};
use crate::models::actor_messages::{ClientActorMessage, Connect, Disconnect, WsMessage};
use crate::utils::enums::ClientMessage;
use crate::utils::enums::ServerMessage;

type Session = Recipient<WsMessage>;

#[derive(Clone)]
pub struct GameServerState {
  pub db: Arc<Mutex<Database>>,
  pub sessions: HashMap<String, Session>,
  pub rooms: HashMap<String, HashSet<String>>,
}

// GameServer actor which keeps track of all the sessions and game rooms (each game room has up to 4 sessions)
pub struct GameServer {
  db: Arc<Mutex<Database>>,
  sessions: HashMap<String, Session>, // player_id => Addres to send messages
  rooms: HashMap<String, HashSet<String>>, // room_id / game_id => player_id
}

impl GameServer {
  pub fn new(db: Arc<Mutex<Database>>) -> Self {
    GameServer {
      db,
      sessions: HashMap::new(),
      rooms: HashMap::new(),
    }
  }

  pub fn get_state(&self) -> GameServerState {
    GameServerState {
      db: self.db.clone(),
      sessions: self.sessions.clone(),
      rooms: self.rooms.clone(),
    }
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
    self.sessions.insert(msg.player_id.clone(), msg.address);
    self
      .rooms
      .entry(msg.room_id.clone())
      .or_insert_with(HashSet::new)
      .insert(msg.player_id.clone());

    let count = self.sessions.len();
    let server_msg = ServerMessage::PlayerCountChange(count);
    let json = serde_json::to_string(&server_msg).unwrap();

    send_message_to_room(
      json.as_str(),
      self.sessions.clone(),
      self.rooms.clone(),
      msg.room_id.as_str(),
    );
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

    let server_msg = ServerMessage::PlayerCountChange(self.sessions.len());
    let json = serde_json::to_string(&server_msg).unwrap();

    for room in rooms {
      send_message_to_room(
        json.as_str(),
        self.sessions.clone(),
        self.rooms.clone(),
        room.as_str(),
      );
    }
  }
}

// #[async_trait]
impl Handler<ClientActorMessage> for GameServer {
  type Result = ();

  fn handle(&mut self, msg: ClientActorMessage, ctx: &mut Context<Self>) {
    let result = serde_json::from_str::<ClientMessage>(msg.content.as_str());

    let message = match result {
      Ok(message) => message,
      // TODO: handle errors [send back Error(String) message ??]
      Err(_) => return,
    };

    let state = self.get_state();
    actix_web::rt::spawn(async move {
      match message {
        ClientMessage::ThrowDice => todo!(),
        ClientMessage::MoveFigure(_, _) => todo!(),
        ClientMessage::PromotePiece => todo!(),
        ClientMessage::StartGame => start_game(state, msg).await,
      };
    });

    // let client_msg = serde_json::from_str(msg.msg.as_str()).unwrap();
    // match client_msg {
    //   ClientMessage::ThrowDice => {
    //     let dice_value = get_dice_value();
    //     let mut game = find_game(&self.db, msg.room_id.as_str()).await.unwrap().unwrap();
    //     game.dice_throws.push(dice_value);
    //     let mut dice_throws_sum = game.dice_throws.iter().sum::<usize>();
    //
    //     if dice_throws_sum == 18 {
    //       // player should be skipped
    //       game.update_current_player();
    //       game.dice_throws.clear();
    //       // TODO: update Game as a struct in DB (attribute dice_throws changed)
    //       ServerMessage::DiceValue(dice_value, false);
    //       ServerMessage::SkipPlayer;
    //       ServerMessage::GameUpdate(game);
    //     }
    //
    //     match dice_value {
    //       6 => {
    //         // has to throw again (a bonus throw)
    //         ServerMessage::DiceValue(dice_value, true);
    //         // update Game as a struct in DB (attribute dice_throws changed)
    //         // sending GameUpdate shouldn't be necessary
    //       },
    //       // player threw 1-5 (doesn't get a bonus throw)
    //       n => {
    //         dice_throws_sum += n;
    //         // TODO: need to implement get_available_positions based on make_a_move_bot()
    //         let available_positions = game.get_available_positions(dice_throws_sum);
    //         // update Game as a struct in DB (attribute dice_throws changed)
    //         ServerMessage::DiceValue(n, false);
    //         ServerMessage::AvailablePositions(available_positions);
    //         // sending GameUpdate shouldn't be necessary
    //       }
    //     }
    //   },
    //
    //   ClientMessage::MoveFigure(position, color) => {
    //     let is_position_in_home = color.is_some();
    //     let move_result = play_round(&self.db,msg.room_id.clone(), position, is_position_in_home).await;
    //
    //     // TODO: either return Game from play_round, or get it from DB
    //     //  also .unwrap().unwrap() is fine here, since game _should_ exist ?
    //     let game = find_game(&self.db, msg.room_id.as_str()).await.unwrap().unwrap();
    //
    //     match move_result {
    //       MoveResult::Winner(winner) => {
    //         // send the following messages to client(s)
    //         ServerMessage::MoveSuccessful("Jumped to finish.".into());
    //         ServerMessage::GameUpdate(game);
    //         ServerMessage::GameOver(winner);
    //       }
    //       MoveResult::Success(_msg) => {
    //         ServerMessage::MoveSuccessful(_msg);
    //         ServerMessage::GameUpdate(game);
    //       }
    //       MoveResult::Error(_msg) => {
    //         ServerMessage::MoveFailed(_msg);
    //       }
    //     }
    //     // after making a successful, check if there is a winner => send GameOver
    //   },
    //
    //   ClientMessage::PromotePiece => {
    //     // set position to PROMOTE_PIECE constant OR change it to bool ('to_promote' / 'should_promote')
    //     // otherwise the same as MovePiece(PROMOTE_PIECE, None) ??
    //   }
    // }

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
