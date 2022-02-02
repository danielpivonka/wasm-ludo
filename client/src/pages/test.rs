use futures::{StreamExt, SinkExt, stream::{SplitSink, SplitStream}};
use reqwasm::websocket::{futures::WebSocket, Message};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

fn main() {
  yew::start_app::<App>();
}

enum Msg {
  Connect,
}

struct App {
  ws: (SplitSink<WebSocket, Message>, SplitStream<WebSocket>),
}

impl Component for App {
  type Message = Msg;
  type Properties = ();

  fn create(_ctx: &Context<Self>) -> Self {
      Self {
          ws: WebSocket::open("ws://127.0.0.1:8000/ws/").unwrap().split(),
      }
  }

  fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
      match msg {
          Msg::Connect => {
              let (mut write, mut read) = self.ws;
              /* error: cannot move out of `self.ws` which is behind a mutable reference
              label: move occurs because `self.ws` has type `reqwasm::websocket::futures::WebSocket`, which does not implement the `Copy` trait */

              spawn_local(async move {
                  write
                      .send(Message::Text(String::from("Websocket connected")))
                      .await
                      .unwrap();
              });
              false
          }
      }
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    html! {
      <>
        <button onclick ={ctx.link().callback(|_| Msg::Connect)}>{"Connect"}</button>
      </>
    }
  }
}

  // {
  //   let id = id.clone();
  //   let sender = sender.clone();
  //   use_effect_with_deps(
  //     move |_: &[u32; 0]| {
  //       let player_id: String = SessionStorage::get("player_id").unwrap();
  //       // let res = Request::get(format!("ws://127.0.0.1:8080/games/websocket/{}/{}", id, player_id)).send().await;
        
  //        let websocket = WebSocket::open(
  //          format!("ws://127.0.0.1:8080/games/websocket/{}/{}", id, player_id).as_str(),
  //        )
  //        .unwrap();
        
  //       // ws.set(Some(websocket.split()));
  //       let ws = WSWrapper{ws:Some(websocket)};
  //       let (tx, rx) = ws.get_channel();
  //       let (tx2, rx2) = ws.get_channel();
  //       sender.set(Some(tx));
  //       move || {}
  //     },
  //     [],
  //   );
  // }
  
  // // TODO: implement use effect
  // {
  //   let sender = sender.clone();
  //   let player_count = player_count.clone();
  //   let id = id.clone();
  //   let history = history.clone();
  //   use_effect_with_deps::<_, Box<dyn FnOnce() -> ()>, _>(
  //     move |_: &[u32; 0]| {
  //       spawn_local(async move {
  //         // TODO: handle errors as well
  //         while let Some(Ok(Message::Text(text))) = ws.as_ref().unwrap().1.next().await {
  //           if let Ok(message) = serde_json::from_str::<ServerMessage>(text.as_str()) {
  //             match message {
  //               ServerMessage::SkipPlayer => todo!(),
  //               ServerMessage::MoveSuccessful(_) => todo!(),
  //               ServerMessage::MoveFailed(_) => todo!(),
  //               ServerMessage::PiecePromoted => todo!(),
  //               ServerMessage::Information(_) => todo!(),
  //               ServerMessage::GameUpdate(_) => todo!(),
  //               ServerMessage::PlayerConnected(_) => todo!(),
  //               ServerMessage::PlayerDisconnected(_) => todo!(),
  //               ServerMessage::PlayerCountChange(players) => {
  //                 player_count.set(players);
  //               }
  //               ServerMessage::GameStarted => {
  //                 history.push(MainRoute::Game { id: id.clone() });
  //               },
  //               ServerMessage::DiceValue(_, _) => todo!(),
  //               ServerMessage::AvailablePositions(_, _, _) => todo!(),
  //               ServerMessage::GameOver(_) => todo!(),
  //               ServerMessage::Error(_) => todo!(),
  //             };
  //             log!(format!("1. {:?}", message))
  //           }
  //         }
  //         log!("WebSocket Closed")
  //       });

  //       spawn_local(async move {
  //         ws.unwrap().0.send(Message::Text("hello".into())).await.unwrap();
  //       });

  //       Box::new(|| {})
  //     },
  //     [],
  //   );
  // }