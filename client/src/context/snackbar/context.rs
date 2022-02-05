use yew::prelude::*;

#[derive(PartialEq, Clone, Debug)]
pub enum SanckbarVariant {
  Success,
  Warning,
  Error,
}

#[derive(PartialEq, Clone, Debug)]
pub struct SnackbarOptions {
  pub message: String,
  pub variant: SanckbarVariant,
}

#[derive(PartialEq, Clone, Debug)]
pub struct SnackbarContext {
  pub open: Callback<SnackbarOptions>,
}
