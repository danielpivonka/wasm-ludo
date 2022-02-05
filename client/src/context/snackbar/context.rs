use yew::prelude::*;

#[derive(PartialEq, Clone, Debug)]
pub enum SnackbarVariant {
  Success,
  Warning,
  Error,
}

#[derive(PartialEq, Clone, Debug)]
pub struct SnackbarOptions {
  pub message: String,
  pub variant: SnackbarVariant,
}

#[derive(PartialEq, Clone, Debug)]
pub struct SnackbarContext {
  pub open: Callback<SnackbarOptions>,
}
