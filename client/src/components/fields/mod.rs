use std::collections::HashMap;
use std::iter::FromIterator;
use std::array::IntoIter;

use yew::prelude::*;

use crate::models::color::Color;
use crate::components::field::Field;
use crate::components::icon::Icon;
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
}


#[function_component(Fields)]
pub fn fields(props: &FieldsProps) -> Html {
  let FieldsProps {color, position, fields} = props.clone();
  
  let left_position_map: HashMap<usize, usize> = HashMap::from_iter(IntoIter::new([
    (17, 0),
    (16, 1),
    (15, 2),
    (14, 3),
    (13, 4),
    (12, 5),
    (6, 6),
    (0, 7),
    (1, 8),
    (2, 9),
    (3, 10),
    (4, 11),
    (5, 12),
  ]));

  let top_position_map: HashMap<usize, usize> = HashMap::from_iter(IntoIter::new([
    (15, 0),
    (12, 1),
    (9, 2),
    (6, 3),
    (3, 4),
    (0, 5),
    (1, 6),
    (2, 7),
    (5, 8),
    (8, 9),
    (11, 10),
    (14, 11),
    (17, 12),
  ]));

  let right_position_map: HashMap<usize, usize> = HashMap::from_iter(IntoIter::new([
    (0, 0),
    (1, 1),
    (2, 2),
    (3, 3),
    (4, 4),
    (5, 5),
    (11, 6),
    (17, 7),
    (16, 8),
    (15, 9),
    (14, 10),
    (13, 11),
    (12, 12),
  ]));

  let bottom_position_map: HashMap<usize, usize> = HashMap::from_iter(IntoIter::new([
    (2, 0),
    (5, 1),
    (8, 2),
    (11, 3),
    (14, 4),
    (17, 5),
    (16, 6),
    (15, 7),
    (12, 8),
    (9, 9),
    (6, 10),
    (3, 11),
    (0, 12),
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

  html! {
    <div class={classes!(String::from("w-full h-full grid"), classes)}>
      {
        fields.iter().enumerate().map(|(index, field)| {
          {
            if let Some(position) = map.get(&index) {
              html! { 
                <Field 
                  color={color.clone()}
                  color_background={*position == 8}
                >
                  {
                    if *position == 6 {
                      html! { <Icon class={classes!(arrow_class.clone())} /> }
                    } else {
                      html! { format!("{}", position) }
                    }
                  }
                </Field>
              }
            } else {
              html! { <Field color={color.clone()} color_background={true} /> }
            }
          }
        }).collect::<Vec<Html>>()
      }
    </div>
  }
}