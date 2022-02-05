use reqwasm::websocket::futures::WebSocket;

pub struct StateWebSocket(pub WebSocket);

impl PartialEq for StateWebSocket {
  fn eq(&self, other: &Self) -> bool {
    true
  }
}
