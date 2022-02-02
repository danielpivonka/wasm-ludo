use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::game::Game;
use crate::pages::game_join::GameJoin;
use crate::pages::game_lobby::GameLobby;
use crate::pages::home::Home;
use crate::pages::not_found::NotFound;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
  #[at("/")]
  Home,
  #[at("/games/:id/lobby")]
  GameLobby { id: String },
  #[at("/games/:id/join")]
  GameJoin { id: String },
  #[at("/games/:id")]
  Game { id: String },
  #[not_found]
  #[at("/404")]
  NotFound,
}

fn switch(routes: &Route) -> Html {
  match routes {
    Route::Home => html! {<Home />},
    Route::GameLobby { id } => html! {<GameLobby id={ id.clone() } />},
    Route::GameJoin { id } => html! {<GameJoin id={ id.clone() } />},
    Route::Game { id } => html! {<Game id={ id.clone() } />},
    Route::NotFound => html! {<NotFound />},
  }
}

#[function_component(Routes)]
pub fn routes() -> Html {
  html! {
    <BrowserRouter>
      <Switch<Route> render={Switch::render(switch)} />
    </BrowserRouter>
  }
}
