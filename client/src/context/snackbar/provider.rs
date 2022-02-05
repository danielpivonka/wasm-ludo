use yew::prelude::*;

use crate::components::icon::Icon;
use crate::context::snackbar::context::{SnackbarContext, SnackbarOptions, SnackbarVariant};

use super::hook::{use_snackbar, UseToastValues};

#[derive(Properties, PartialEq, Clone)]
pub struct SnackbarProviderProps {
  #[prop_or_default]
  pub children: Children,
}

#[function_component(SnackbarProvider)]
pub fn snackbar_provider(props: &SnackbarProviderProps) -> Html {
  let UseToastValues {
    open,
    is_open,
    options,
    close,
  } = use_snackbar();
  let SnackbarOptions { message, variant } = options;

  let context = SnackbarContext { open };
  let variant_class = match variant {
    SnackbarVariant::Success => "text-green-600",
    SnackbarVariant::Warning => "text-yellow-600",
    SnackbarVariant::Error => "text-red-600",
  };

  let icon = match variant {
    SnackbarVariant::Success => html! { <Icon class="fas fa-check" /> },
    SnackbarVariant::Warning => html! { <Icon class="fas fa-exclamation" /> },
    SnackbarVariant::Error => html! { <Icon class="fas fa-bug" /> },
  };

  let onclick = Callback::from(move |_| {
    close.emit(());
  });

  html! {
    <ContextProvider<SnackbarContext> context={context}>
      { for props.children.iter() }
      <div class={classes!(String::from("fixed rounded border-2 shadow-2xl bg-neutral-50 border-neutral-300 left-5 bottom-5 p-3"), (!is_open).then(|| "hidden"))}>
        <div class="absolute top-0 right-2">
          <Icon class="text-sm fas fa-times text-neutral-600" {onclick} />
        </div>
        <div class="flex items-center">
          <div class={variant_class}>{icon}</div>
          <span class="ml-4 text-md font-semibold mr-6">{ message.clone() }</span>
        </div>
      </div>
    </ContextProvider<SnackbarContext>>
  }
}
