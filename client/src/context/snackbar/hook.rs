use gloo::timers::callback::Timeout;
use yew::prelude::*;

use super::context::{SnackbarOptions, ToastType};

#[derive(Clone, Debug, PartialEq)]
pub struct UseToastValues {
  pub open: Callback<SnackbarOptions>,
  pub close: Callback<()>,
  pub is_open: bool,
  pub options: SnackbarOptions,
}

pub fn use_snackbar() -> UseToastValues {
  let is_open = use_state(|| false);
  let options = use_state(|| SnackbarOptions {
    message: "".into(),
    toast_type: ToastType::Success,
  });

  let close = {
    let is_open = is_open.clone();
    Callback::from(move |_| {
      is_open.set(false);
    })
  };

  let open = {
    let is_open = is_open.clone();
    let options = options.clone();
    Callback::from(move |snackbar_opts: SnackbarOptions| {
      is_open.set(true);
      options.set(snackbar_opts);
    })
  };

  {
    let is_open = (*is_open).clone();
    let close = close.clone();
    use_effect_with_deps::<_, Box<dyn FnOnce()>, _>(
      move |is_open| {
        if !is_open {
          return Box::new(|| {});
        }

        let timeout = Timeout::new(5000, move || {
          close.emit(());
        });

        Box::new(|| drop(timeout))
      },
      is_open,
    );
  }

  UseToastValues {
    open,
    is_open: (*is_open).clone(),
    options: (*options).clone(),
    close,
  }
}
