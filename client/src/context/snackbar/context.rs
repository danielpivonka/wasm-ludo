use yew::prelude::*;

#[derive(PartialEq, Clone, Debug)]
pub enum ToastType {
  Success,
  Warning,
  Error,
}

#[derive(PartialEq, Clone, Debug)]
pub struct SnackbarOptions {
  pub message: String,
  pub toast_type: ToastType,
}

#[derive(PartialEq, Clone, Debug)]
pub struct SnackbarContext {
  pub open: Callback<SnackbarOptions>,
}
