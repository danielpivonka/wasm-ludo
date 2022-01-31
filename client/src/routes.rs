use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::game_lobby::GameLobby;
use crate::pages::not_found::NotFound;
use crate::pages::game::Game;
use crate::pages::home::Home;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
  #[at("/")]
  Home,
  #[at("/game/:id/lobby")]
  GameLobby { id: String },
  #[at("/game/:id")]
  Game { id: String },
  #[not_found]
  #[at("/404")]
  NotFound,
}

fn switch(routes: &Route) -> Html {
  match routes {
    Route::Home => html! {<Home />},
    Route::GameLobby { id } => html! {<GameLobby id={ id.clone() } />},
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
