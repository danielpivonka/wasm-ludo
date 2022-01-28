#[macro_export]
macro_rules! classnames {
  ($class:expr => $condition:expr) => {{
    if $condition {
      format!("{}", $class)
    } else {
      String::from("")
    }
  }};
  ($class:expr) => {
    format!("{}", $class)
  };
  ($class:expr => $condition:expr, $($rest:tt)+) => {{
    if $condition {
      format!("{} {}", $class, classnames!($($rest)+))
    } else {
      classnames!($($rest)+)
    }
  }};
  ($class:expr, $($rest:tt)+) => {
    format!("{} {}", $class, classnames!($($rest)+))
  };
}

pub use classnames;