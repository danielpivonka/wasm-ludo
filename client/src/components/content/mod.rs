use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct ContentProps {
  #[prop_or_default]
  pub children: Children,
  #[prop_or(String::from(""))]
  pub class: String,
}

#[function_component(Content)]
pub fn content(props: &ContentProps) -> Html {
  let ContentProps { children, class } = props;

  html! {
    <div class={classes!(String::from("max-w-screen-xl mx-auto px-2 md:px-4"), class)}>
      {for children.iter()}
    </div>
  }
}
