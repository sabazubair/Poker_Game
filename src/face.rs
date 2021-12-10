use std::fmt;
// use std::cmp::Ordering;
use std::str::FromStr;

#[derive(Clone, Debug, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub enum Face {
  Two,
  Three,
  Four,
  Five,
  Six,
  Seven,
  Eight,
  Nine,
  Ten,
  Jack,
  Queen,
  King,
  Ace,
}

impl Face {
  pub fn is_ace(&self) -> bool {
    self == &Self::Ace
  }
  pub fn counters() -> [(u8, Self); 13] {
    [
      (0, Self::Two),
      (0, Self::Three),
      (0, Self::Four),
      (0, Self::Five),
      (0, Self::Six),
      (0, Self::Seven),
      (0, Self::Eight),
      (0, Self::Nine),
      (0, Self::Ten),
      (0, Self::Jack),
      (0, Self::Queen),
      (0, Self::King),
      (0, Self::Ace),
    ]
  }
}

// impl Ord for Face {
//   fn cmp(&self, other: &Self) -> Ordering {

//   }
// }

impl fmt::Display for Face {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Self::Ace => "1",
        Self::Two => "2",
        Self::Three => "3",
        Self::Four => "4",
        Self::Five => "5",
        Self::Six => "6",
        Self::Seven => "7",
        Self::Eight => "8",
        Self::Nine => "9",
        Self::Ten => "10",
        Self::Jack => "11",
        Self::Queen => "12",
        Self::King => "13",
      }
    )
  }
}

impl FromStr for Face {
  type Err = &'static str;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "A" => Ok(Self::Ace),
      "2" => Ok(Self::Two),
      "3" => Ok(Self::Three),
      "4" => Ok(Self::Four),
      "5" => Ok(Self::Five),
      "6" => Ok(Self::Six),
      "7" => Ok(Self::Seven),
      "8" => Ok(Self::Eight),
      "9" => Ok(Self::Nine),
      "T" => Ok(Self::Ten),
      "J" => Ok(Self::Jack),
      "Q" => Ok(Self::Queen),
      "K" => Ok(Self::King),
      _ => Err("Invalid face."),
    }
  }
}

impl From<u32> for Face {
  fn from(idx: u32) -> Self {
    match idx {
      0 => Self::Ace,
      1 => Self::Two,
      2 => Self::Three,
      3 => Self::Four,
      4 => Self::Five,
      5 => Self::Six,
      6 => Self::Seven,
      7 => Self::Eight,
      8 => Self::Nine,
      9 => Self::Ten,
      10 => Self::Jack,
      11 => Self::Queen,
      12 => Self::King,
      _ => panic!("Invalid face"),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_ord() {
    assert!(Face::Ten > Face::Five);
    assert!(Face::Ace > Face::King);
  }
}
