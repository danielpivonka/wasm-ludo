use yew::prelude::*;
use stylist::{yew::styled_component, css};

#[derive(Properties, PartialEq, Clone)]
pub struct BoardMiddleProps {}

#[styled_component(BoardMiddle)]
pub fn board_middle(props: &BoardMiddleProps) -> Html {
  html! {
    <div class="h-full w-full relative">
      <div class={classes!(String::from("absolute top-0 left-0 right-0 bg-blue-400 h-1/2 w-full"), css!("clip-path: polygon(0% 0%, 100% 0%, 50% 100%);"))} />
      <div class={classes!(String::from("absolute bottom-0 left-0 right-0 bg-green-400 h-1/2 w-full"), css!("clip-path: polygon(0% 100%, 50% 0%, 100% 100%);"))} />
      <div class={classes!(String::from("absolute top-0 bottom-0 left-1/2 bg-red-400 w-1/2 h-full"), css!("clip-path: polygon(100% 0%, 0% 50%, 100% 100%);"))} />
      <div class={classes!(String::from("absolute top-0 bottom-0 left-0 bg-yellow-400 w-1/2 h-full"), css!("clip-path: polygon(0% 0%, 100% 50%, 0% 100%);"))} />
    </div>
  }
}