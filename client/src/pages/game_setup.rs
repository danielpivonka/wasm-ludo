use yew::prelude::*;

use crate::components::content::Content;

#[derive(Properties, PartialEq, Clone)]
pub struct GameSetupProps {}

#[function_component(GameSetup)]
pub fn game_setup(props: &GameSetupProps) -> Html {

  html! {
    <Content>
      <div class="flex items-center">
        <div class="flex flex-col">
          <p>{"Ludo"}</p>
        </div>
      </div>
    </Content>
  }
}