use std::fmt;
use std::str::FromStr;

#[derive(Clone, Debug, Copy, Eq, PartialEq)]
pub enum Suit {
  Clubs,
  Diamonds,
  Hearts,
  Spades,
}

impl fmt::Display for Suit {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Self::Clubs => "C",
        Self::Diamonds => "D",
        Self::Hearts => "H",
        Self::Spades => "S",
      }
    )
  }
}

impl From<i32> for Suit {
  fn from(idx: i32) -> Self {
    match idx {
      0 => Self::Clubs,
      1 => Self::Diamonds,
      2 => Self::Hearts,
      3 => Self::Spades,
      _ => panic!("Invalid suit"),
    }
  }
}

impl FromStr for Suit {
  type Err = &'static str;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "C" => Ok(Self::Clubs),
      "D" => Ok(Self::Diamonds),
      "H" => Ok(Self::Hearts),
      "S" => Ok(Self::Spades),
      _ => Err("Invalid suit."),
    }
  }
}
