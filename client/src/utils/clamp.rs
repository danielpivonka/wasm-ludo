use std::cmp::{max, min};

pub fn clamp<T>(value: T, min_value: T, max_value: T) -> T
where
  T: Ord,
{
  max(min(value, max_value), min_value)
}
