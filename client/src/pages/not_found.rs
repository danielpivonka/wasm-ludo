use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::button::Button;
use crate::components::card::Card;
use crate::components::content::Content;
use crate::components::icon::Icon;

use crate::routes::GameRoute;

#[function_component(NotFound)]
pub fn not_found() -> Html {
  let history = use_history().unwrap();
  let onclick = Callback::once(move |_| {
    history.push(GameRoute::GameLobby {
      id: "mock_id".into(),
    })
  });

  let icon = html! {
    <Icon class="fas fa-home" />
  };

  html! {
    <Content class="h-full grid place-items-center">
      <Card class="p-4 mx-6 md:mx-0 md:p-24 w-full grid place-items-center gap-10">
        <div class="flex items-center justify-center gap-10">
          <Icon class="fas fa-bug text-primary-600 text-8xl" />
          <span class="text-8xl font-bold text-neutral-800">{"404"}</span>
        </div>
        <div class="flex flex-col items-center gap-2">
          <p class="text-4xl font-bold text-neutral-800">{"Page not found"}</p>
          <p class="text-2xl font-semibold text-neutral-600">{"We were unable to find the resource you are looking for"}</p>
        </div>
        <Button {onclick} {icon}>{"Go back to homepage"}</Button>
      </Card>
    </Content>
  }
}
