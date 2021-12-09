use std::fmt;
use std::str::FromStr;

use crate::Face;
use crate::Suit;

#[derive(Clone, Debug, Copy, Eq, PartialEq)]
pub struct Card {
  suit: Suit,
  face: Face,
}
impl Card {
  pub fn suit(&self) -> Suit {
    self.suit
  }
  pub fn face(&self) -> Face {
    self.face
  }
}
impl fmt::Display for Card {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}{}", self.face, self.suit)
  }
}

impl From<u32> for Card {
  fn from(idx: u32) -> Self {
    Self {
      suit: ((idx - 1) / 13).into(),
      face: ((idx - 1) % 13).into(),
    }
  }
}

impl FromStr for Card {
  type Err = &'static str;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(Self {
      face: (&s[0..1]).parse().unwrap(),
      suit: (&s[1..2]).parse().unwrap(),
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_from_int() {
    let card: Card = 16.into();
    assert_eq!(card.to_string(), "3D");
    let card: Card = 49.into();
    assert_eq!(card.to_string(), "TS");
  }
  #[test]
  fn test_from_str() {
    let card: Card = "3D".parse().unwrap();
    assert_eq!(Card::from(16), card);
    let card: Card = "QH".parse().unwrap();
    assert_eq!(Card::from(38), card);
  }
}
