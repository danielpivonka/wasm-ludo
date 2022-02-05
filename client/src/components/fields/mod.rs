use std::array::IntoIter;
use std::collections::HashMap;
use std::iter::FromIterator;

use yew::prelude::*;

use crate::components::field::{Field, FieldVariant};
use crate::models::color::Color;
use crate::types::FieldType;

#[derive(PartialEq, Clone)]
pub enum FieldsPosition {
  Top,
  Right,
  Bottom,
  Left,
}

#[derive(Properties, PartialEq, Clone)]
pub struct FieldsProps {
  pub position: FieldsPosition,
  pub color: Color,
  pub fields: Vec<FieldType>,
  #[prop_or(0)]
  pub offset: usize,
}

#[function_component(Fields)]
pub fn fields(props: &FieldsProps) -> Html {
  let FieldsProps {
    color,
    position,
    fields,
    offset,
  } = props.clone();

  let left_position_map: HashMap<usize, (usize, FieldVariant)> =
    HashMap::from_iter(IntoIter::new([
      (17, (0, FieldVariant::Main)),
      (16, (1, FieldVariant::Main)),
      (15, (2, FieldVariant::Main)),
      (14, (3, FieldVariant::Main)),
      (13, (4, FieldVariant::Main)),
      (12, (5, FieldVariant::Main)),
      (6, (6, FieldVariant::Main)),
      (0, (7, FieldVariant::Main)),
      (1, (8, FieldVariant::Main)),
      (2, (9, FieldVariant::Main)),
      (3, (10, FieldVariant::Main)),
      (4, (11, FieldVariant::Main)),
      (5, (12, FieldVariant::Main)),
      (7, (0, FieldVariant::Home)),
      (8, (1, FieldVariant::Home)),
      (9, (2, FieldVariant::Home)),
      (10, (3, FieldVariant::Home)),
      (11, (4, FieldVariant::Home)),
    ]));

  let top_position_map: HashMap<usize, (usize, FieldVariant)> =
    HashMap::from_iter(IntoIter::new([
      (15, (0, FieldVariant::Main)),
      (12, (1, FieldVariant::Main)),
      (9, (2, FieldVariant::Main)),
      (6, (3, FieldVariant::Main)),
      (3, (4, FieldVariant::Main)),
      (0, (5, FieldVariant::Main)),
      (1, (6, FieldVariant::Main)),
      (2, (7, FieldVariant::Main)),
      (5, (8, FieldVariant::Main)),
      (8, (9, FieldVariant::Main)),
      (11, (10, FieldVariant::Main)),
      (14, (11, FieldVariant::Main)),
      (17, (12, FieldVariant::Main)),
      (4, (0, FieldVariant::Home)),
      (7, (1, FieldVariant::Home)),
      (10, (2, FieldVariant::Home)),
      (13, (3, FieldVariant::Home)),
      (16, (4, FieldVariant::Home)),
    ]));

  let right_position_map: HashMap<usize, (usize, FieldVariant)> =
    HashMap::from_iter(IntoIter::new([
      (0, (0, FieldVariant::Main)),
      (1, (1, FieldVariant::Main)),
      (2, (2, FieldVariant::Main)),
      (3, (3, FieldVariant::Main)),
      (4, (4, FieldVariant::Main)),
      (5, (5, FieldVariant::Main)),
      (11, (6, FieldVariant::Main)),
      (17, (7, FieldVariant::Main)),
      (16, (8, FieldVariant::Main)),
      (15, (9, FieldVariant::Main)),
      (14, (10, FieldVariant::Main)),
      (13, (11, FieldVariant::Main)),
      (12, (12, FieldVariant::Main)),
      (6, (4, FieldVariant::Home)),
      (7, (3, FieldVariant::Home)),
      (8, (2, FieldVariant::Home)),
      (9, (1, FieldVariant::Home)),
      (10, (0, FieldVariant::Home)),
    ]));

  let bottom_position_map: HashMap<usize, (usize, FieldVariant)> =
    HashMap::from_iter(IntoIter::new([
      (2, (0, FieldVariant::Main)),
      (5, (1, FieldVariant::Main)),
      (8, (2, FieldVariant::Main)),
      (11, (3, FieldVariant::Main)),
      (14, (4, FieldVariant::Main)),
      (17, (5, FieldVariant::Main)),
      (16, (6, FieldVariant::Main)),
      (15, (7, FieldVariant::Main)),
      (12, (8, FieldVariant::Main)),
      (9, (9, FieldVariant::Main)),
      (6, (10, FieldVariant::Main)),
      (3, (11, FieldVariant::Main)),
      (0, (12, FieldVariant::Main)),
      (1, (4, FieldVariant::Home)),
      (4, (3, FieldVariant::Home)),
      (7, (2, FieldVariant::Home)),
      (10, (1, FieldVariant::Home)),
      (13, (0, FieldVariant::Home)),
    ]));

  let (map, arrow_class): (_, String) = match position {
    FieldsPosition::Top => (top_position_map, "fas fa-long-arrow-alt-down".into()),
    FieldsPosition::Right => (right_position_map, "fas fa-long-arrow-alt-left".into()),
    FieldsPosition::Bottom => (bottom_position_map, "fas fa-long-arrow-alt-up".into()),
    FieldsPosition::Left => (left_position_map, "fas fa-long-arrow-alt-right".into()),
  };

  let classes: String = match position {
    FieldsPosition::Top | FieldsPosition::Bottom => "grid-cols-3 grid-rows-6".into(),
    FieldsPosition::Right | FieldsPosition::Left => "grid-cols-6 grid-rows-3".into(),
  };

  let fields = fields.iter().enumerate().map(|(index, _)| {
    if let Some((raw_position, variant)) = map.get(&index) {
      let position = match variant {
        FieldVariant::Home => *raw_position,
        FieldVariant::Main => *raw_position + offset,
      };
      html! {
        <Field
          color={color.clone()}
          color_background={*raw_position == 8 || *variant == FieldVariant::Home}
          {position}
          raw_position={*raw_position}
          variant={variant.clone()}
          arrow_class={arrow_class.clone()}
        />
      }
    } else {
      html! {}
    }
  });

  html! {
    <div class={classes!(String::from("w-full h-full grid"), classes)}>
      { for fields }
    </div>
  }
}
