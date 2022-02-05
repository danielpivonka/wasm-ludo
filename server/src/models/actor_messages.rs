use actix::{Message, Recipient};

// `rtype` is a return type of the message

#[derive(Message)]
#[rtype(result = "()")]
pub struct WsMessage(pub String);

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
  pub address: Recipient<WsMessage>,
  pub player_id: String,
  pub room_id: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
  pub player_id: String,
  pub room_id: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientActorMessage {
  pub content: String,
  pub room_id: String,
  pub player_id: String,
}
