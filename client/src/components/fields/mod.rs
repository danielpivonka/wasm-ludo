use std::array::IntoIter;
use std::collections::HashMap;
use std::iter::FromIterator;

use yew::prelude::*;

use crate::components::field::Field;
use crate::components::icon::Icon;
use crate::components::pawn::Pawn;
use crate::context::game_context::model::GameContext;
use crate::models::color::Color;
use crate::types::FieldType;

#[derive(PartialEq, Clone)]
pub enum FieldsPosition {
  Top,
  Right,
  Bottom,
  Left,
}

#[derive(PartialEq)]
enum FP {
  Home,
  Main
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
  let GameContext { game, .. } = use_context::<GameContext>().expect("context not found");
  let FieldsProps {
    color,
    position,
    fields,
    offset,
  } = props.clone();

  let left_position_map: HashMap<usize, (usize, FP)> = HashMap::from_iter(IntoIter::new([
    (17, (0, FP::Main)),
    (16, (1, FP::Main)),
    (15, (2, FP::Main)),
    (14, (3, FP::Main)),
    (13, (4, FP::Main)),
    (12, (5, FP::Main)),
    (6, (6, FP::Main)),
    (0, (7, FP::Main)),
    (1, (8, FP::Main)),
    (2, (9, FP::Main)),
    (3, (10, FP::Main)),
    (4, (11, FP::Main)),
    (5, (12, FP::Main)),
    (7, (0, FP::Home)),
    (8, (1, FP::Home)),
    (9, (2, FP::Home)),
    (10, (3, FP::Home)),
    (11, (4, FP::Home)),
  ]));

  let top_position_map: HashMap<usize, (usize, FP)> = HashMap::from_iter(IntoIter::new([
    (15, (0, FP::Main)),
    (12, (1, FP::Main)),
    (9, (2, FP::Main)),
    (6, (3, FP::Main)),
    (3, (4, FP::Main)),
    (0, (5, FP::Main)),
    (1, (6, FP::Main)),
    (2, (7, FP::Main)),
    (5, (8, FP::Main)),
    (8, (9, FP::Main)),
    (11, (10, FP::Main)),
    (14, (11, FP::Main)),
    (17, (12, FP::Main)),
    (4, (0, FP::Home)),
    (7, (1, FP::Home)),
    (10, (2, FP::Home)),
    (13, (3, FP::Home)),
    (16, (4, FP::Home)),
  ]));

  let right_position_map: HashMap<usize, (usize, FP)> = HashMap::from_iter(IntoIter::new([
    (0, (0, FP::Main)),
    (1, (1, FP::Main)),
    (2, (2, FP::Main)),
    (3, (3, FP::Main)),
    (4, (4, FP::Main)),
    (5, (5, FP::Main)),
    (11, (6, FP::Main)),
    (17, (7, FP::Main)),
    (16, (8, FP::Main)),
    (15, (9, FP::Main)),
    (14, (10, FP::Main)),
    (13, (11, FP::Main)),
    (12, (12, FP::Main)),
    (6, (4, FP::Home)),
    (7, (3, FP::Home)),
    (8, (2, FP::Home)),
    (9, (1, FP::Home)),
    (10, (0, FP::Home)),
  ]));

  let bottom_position_map: HashMap<usize, (usize, FP)> = HashMap::from_iter(IntoIter::new([
    (2, (0, FP::Main)),
    (5, (1, FP::Main)),
    (8, (2, FP::Main)),
    (11, (3, FP::Main)),
    (14, (4, FP::Main)),
    (17, (5, FP::Main)),
    (16, (6, FP::Main)),
    (15, (7, FP::Main)),
    (12, (8, FP::Main)),
    (9, (9, FP::Main)),
    (6, (10, FP::Main)),
    (3, (11, FP::Main)),
    (0, (12, FP::Main)),
    (1, (4, FP::Home)),
    (4, (3, FP::Home)),
    (7, (2, FP::Home)),
    (10, (1, FP::Home)),
    (13, (0, FP::Home)),
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
        fields.iter().enumerate().map(|(index, _field)| {
          {
            if let Some((raw_pos, field_type)) = map.get(&index) {
              let position = raw_pos + offset;
              let pawn_color = game.clone().and_then(|game| {
                if *field_type == FP::Home {
                  // TODO: add home pawns
                  None
                } else {
                  game.fields.get(position).unwrap_or(&None).clone()
                }
              });
              
              html! {
                <Field
                  color={color.clone()}
                  color_background={*raw_pos == 8 || *field_type == FP::Home}
                >
                  {
                    if *raw_pos == 6 {
                      html! { <Icon class={classes!(arrow_class.clone())} /> }
                    } else if let Some(color) = pawn_color {
                      html! { <Pawn {color} /> }
                    } else {
                      html! {}
                    }
                  }
                </Field>
              }
            } else {
              html! { <Field color={color.clone()} color_background={true}>{format!("{}", index)}</Field> }
            }
          }
        }).collect::<Vec<Html>>()
      }
    </div>
  }
}
