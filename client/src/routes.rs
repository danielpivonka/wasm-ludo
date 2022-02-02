use yew::prelude::*;
use yew_router::prelude::*;

use crate::context::game_context::provider::GameProvider;
use crate::pages::game::Game;
use crate::pages::game_join::GameJoin;
use crate::pages::game_lobby::GameLobby;
use crate::pages::home::Home;
use crate::pages::not_found::NotFound;

#[derive(Clone, Routable, PartialEq)]
pub enum MainRoute {
  #[at("/")]
  Home,
  #[at("/games/:id/join")]
  GameJoin { id: String },
  #[at("/games/:id/:rest")]
  Game { id: String },
  #[not_found]
  #[at("/404")]
  NotFound,
}

#[derive(Clone, Routable, PartialEq)]
pub enum GameRoute {
  #[at("/games/:id/lobby")]
  GameLobby { id: String },
  #[at("/games/:id")]
  Game { id: String },
}

fn switch_game(route: &GameRoute) -> Html {
  match route {
    GameRoute::GameLobby { id } => html! { <GameLobby id={ id.clone() } /> },
    GameRoute::Game { id } => html! { <Game id={ id.clone() } /> },
  }
}

fn switch_main(routes: &MainRoute) -> Html {
  match routes {
    MainRoute::Home => html! {<Home />},
    MainRoute::GameJoin { id } => html! { <GameJoin id={ id.clone() } /> },
    MainRoute::Game { id } => html! {
      <GameProvider game_id={ id.clone() }>
        <Switch<GameRoute> render={Switch::render(switch_game)} />
      </GameProvider>
    },
    MainRoute::NotFound => html! {<NotFound />},
  }
}

#[function_component(Routes)]
pub fn routes() -> Html {
  html! {
    <BrowserRouter>
      <Switch<MainRoute> render={Switch::render(switch_main)} />
    </BrowserRouter>
  }
}
