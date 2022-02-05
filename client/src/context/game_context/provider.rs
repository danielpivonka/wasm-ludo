use yew::prelude::*;

use super::{
  context::GameContext,
  hook::{use_game, UseGameProps},
};

#[derive(Properties, PartialEq, Clone)]
pub struct GameProviderProps {
  #[prop_or_default]
  pub children: Children,
  pub game_id: String,
}

#[function_component(GameProvider)]
pub fn game_provider(props: &GameProviderProps) -> Html {
  let context = use_game(&UseGameProps {
    game_id: props.game_id.clone(),
  });

  html! {
    <ContextProvider<GameContext> context={context}>
      { for props.children.iter() }
    </ContextProvider<GameContext>>
  }
}
