use crate::Card;
use std::str::FromStr;

#[derive(Clone, Debug, Copy, Eq, PartialEq)]
pub struct Hand {
  cards: [Card; 5],
}

impl Hand {
  pub fn new(cards: &[Card; 5]) -> Self {
    Self {
      cards: cards.clone(),
    }
  }
  pub fn cards(&self) -> &[Card; 5] {
    &self.cards
  }
}

impl FromStr for Hand {
  type Err = &'static str;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(Self {
      cards: [
        (&s[0..2]).parse().unwrap(),
        (&s[3..5]).parse().unwrap(),
        (&s[6..8]).parse().unwrap(),
        (&s[9..11]).parse().unwrap(),
        (&s[12..14]).parse().unwrap(),
      ],
    })
  }
}
