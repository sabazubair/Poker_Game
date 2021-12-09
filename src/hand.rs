use crate::Card;
use std::fmt;
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
  pub fn counts(&self) -> ([u8; 13], [u8; 4]) {
    let mut faces: [u8; 13] = [0; 13];
    let mut suits: [u8; 4] = [0; 4];
    for card in &self.cards {
      faces[card.face() as usize] += 1;
      suits[card.suit() as usize] += 1;
    }
    (faces, suits)
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

impl fmt::Display for Hand {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "{} {} {} {} {}",
      self.cards[0], self.cards[1], self.cards[2], self.cards[3], self.cards[4],
    )
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_counts() {
    let hand: Hand = "AH AD AC KS QH".parse().unwrap();
    assert_eq!(
      hand.counts(),
      ([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 3], [1, 1, 2, 1]),
    );
  }
  #[test]
  fn test_display() {
    let hand: Hand = "AH AD AC KS QH".parse().unwrap();
    assert_eq!(hand.to_string(), "AH AD AC KS QH");
  }
}
