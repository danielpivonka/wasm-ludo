use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::game_setup::GameSetup;
use crate::pages::not_found::NotFound;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
  #[at("/")]
  GameSetup,
  #[not_found]
  #[at("/404")]
  NotFound,
}

fn switch(routes: &Route) -> Html {
  match routes {
    Route::GameSetup => html! {<GameSetup />},
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
