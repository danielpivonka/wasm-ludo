use yew::prelude::*;

use crate::components::button::Button;
use crate::components::icon::Icon;
use crate::context::snackbar::context::{SnackbarContext, SnackbarOptions, ToastType};

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
  let SnackbarOptions {
    message,
    toast_type,
  } = options;

  let context = SnackbarContext { open };
  let variant_class = match toast_type {
    ToastType::Success => "text-green-600",
    ToastType::Warning => "text-yellow-600",
    ToastType::Error => "text-red-600",
  };

  let icon = match toast_type {
    ToastType::Success => html! { <Icon class="fas fa-check" /> },
    ToastType::Warning => html! { <Icon class="fas fa-exclamation" /> },
    ToastType::Error => html! { <Icon class="fas fa-bug" /> },
  };

  let onclick = Callback::from(move |_| {
    close.emit(());
  });

  html! {
    <ContextProvider<SnackbarContext> context={context}>
      { for props.children.iter() }
      <div class={classes!(String::from("absolute rounded border-2 shadow-2xl bg-neutral-50 border-neutral-300 left-5 bottom-5 p-3"), (!is_open).then(|| "hidden"))}>
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
