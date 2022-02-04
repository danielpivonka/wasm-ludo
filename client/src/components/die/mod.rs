use yew::prelude::*;

use crate::components::icon::Icon;
use gloo::timers::callback::Timeout;

#[derive(Properties, PartialEq, Clone)]
pub struct DieProps {
  pub number: usize,
  #[prop_or(false)]
  pub is_rolling: bool,
}

const SPIN_TIME: u32 = 1000;

#[function_component(Die)]
pub fn die(props: &DieProps) -> Html {
  let DieProps {
    is_rolling,
    number: prop_number,
  } = props.clone();
  let number = use_state(|| prop_number);
  // let spinning = use_state(|| false);
  // let is_mount = use_mut_ref(|| true);

  {
    let number = number.clone();
    use_effect_with_deps::<_, Box<dyn FnOnce()>, _>(
      move |_| {
        let number_change_timeout = Timeout::new(SPIN_TIME / 2, move || {
          number.set(prop_number);
        });

        Box::new(move || {
          drop(number_change_timeout);
        })
      },
      [prop_number],
    );
  }

  let classes: String = match *number {
    1 => "fa-dice-one".into(),
    2 => "fa-dice-two".into(),
    3 => "fa-dice-three".into(),
    4 => "fa-dice-four".into(),
    5 => "fa-dice-five".into(),
    _ => "fa-dice-six".into(),
  };

  html! {
    <Icon class={classes!(String::from("fas text-4xl text-neutral-600"), classes, is_rolling.then(|| "animate-spin"))} />
  }
}
