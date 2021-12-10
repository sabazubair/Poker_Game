use crate::Card;
use crate::Face;
use crate::Rank;
use crate::Suit;
use std::cmp::Ordering;
use std::fmt;
use std::str::FromStr;

#[derive(Clone, Debug, Copy, Eq, PartialEq)]
pub struct Hand {
  cards: [Card; 5],
}

impl PartialOrd for Hand {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    self.rank().partial_cmp(&other.rank())
  }
}

impl Ord for Hand {
  fn cmp(&self, other: &Self) -> Ordering {
    self.rank().cmp(&other.rank())
  }
}

impl Hand {
  pub fn new(cards: &[Card]) -> Self {
    let mut cards: [Card; 5] = cards.try_into().unwrap();
    cards.sort();
    cards.reverse();
    Self { cards }
  }
  pub fn cards(&self) -> &[Card; 5] {
    &self.cards
  }
  pub fn face_indexes(&self) -> [u8; 5] {
    // converts card to relevant face indexes
    self.cards.map(|c| c.face() as u8)
  }
  pub fn face_offsets(&self) -> [u8; 5] {
    let indexes = self.face_indexes();
    indexes.map(|c| c - indexes[4])
  }
  pub fn is_straight(&self) -> bool {
    match self.face_offsets() {
      [12, 3, 2, 1, 0] => true,
      [4, 3, 2, 1, 0] => true,
      _ => false,
    }
  }
  pub fn counts(&self) -> ([(u8, Face); 5], [(u8, Suit); 4], bool) {
    let mut faces = Face::counters();
    let mut suits = Suit::counters();
    for card in &self.cards {
      faces[card.face() as usize].0 += 1;
      suits[card.suit() as usize].0 += 1;
    }
    faces.sort();
    faces.reverse();
    suits.sort();
    suits.reverse();
    if faces[0].1 == Face::Ace && self.is_straight() && faces[1].1 != Face::King {
      faces.rotate_left(1);
    }
    (faces[0..5].try_into().unwrap(), suits, self.is_straight())
  }
  pub fn rank(&self) -> Rank {
    self.into()
  }
  pub fn as_str_vec(&self) -> Vec<String> {
    self.cards.map(|c| c.to_string()).to_vec()
  }
}

impl FromStr for Hand {
  type Err = &'static str;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut cards = [
      (&s[0..2]).parse().unwrap(),
      (&s[3..5]).parse().unwrap(),
      (&s[6..8]).parse().unwrap(),
      (&s[9..11]).parse().unwrap(),
      (&s[12..14]).parse().unwrap(),
    ];
    cards.sort();
    cards.reverse();
    Ok(Self { cards })
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
  fn test_face_offsets() {
    let hand: Hand = "AH 5C 4D 3S 2H".parse().unwrap();
    assert_eq!(hand.face_offsets(), [12, 3, 2, 1, 0]);
    let hand: Hand = "TH 9C 8D 7S 6H".parse().unwrap();
    assert_eq!(hand.face_offsets(), [4, 3, 2, 1, 0]);
    let hand: Hand = "TH 9C 9D 7S 6H".parse().unwrap();
    assert_eq!(hand.face_offsets(), [4, 3, 3, 1, 0]);
  }
  #[test]
  fn test_is_straight() {
    let hand: Hand = "AH 5C 4D 3S 2H".parse().unwrap();
    assert!(hand.is_straight());
    let hand: Hand = "TH 9C 8D 7S 6H".parse().unwrap();
    assert!(hand.is_straight());
    let hand: Hand = "TH 9C 9D 7S 6H".parse().unwrap();
    assert!(!hand.is_straight());
  }

  #[test]
  fn test_counts() {
    let hand: Hand = "AH AD AC KS QH".parse().unwrap();
    assert_eq!(
      hand.counts(),
      (
        [
          (3, Face::Ace),
          (1, Face::King),
          (1, Face::Queen),
          (0, Face::Jack),
          (0, Face::Ten),
        ],
        [
          (2, Suit::Hearts),
          (1, Suit::Spades),
          (1, Suit::Diamonds),
          (1, Suit::Clubs),
        ],
        false
      ),
    );
    let hand: Hand = "AH KC QD 9S 7H".parse().unwrap();
    assert_eq!(
      hand.counts(),
      (
        [
          (1, Face::Ace),
          (1, Face::King),
          (1, Face::Queen),
          (1, Face::Nine),
          (1, Face::Seven),
        ],
        [
          (2, Suit::Hearts),
          (1, Suit::Spades),
          (1, Suit::Diamonds),
          (1, Suit::Clubs),
        ],
        false
      ),
    );
    let hand: Hand = "TH 9C 8D 7S 6H".parse().unwrap();
    assert_eq!(
      hand.counts(),
      (
        [
          (1, Face::Ten),
          (1, Face::Nine),
          (1, Face::Eight),
          (1, Face::Seven),
          (1, Face::Six),
        ],
        [
          (2, Suit::Hearts),
          (1, Suit::Spades),
          (1, Suit::Diamonds),
          (1, Suit::Clubs),
        ],
        true
      ),
    );
    let hand: Hand = "AH 5C 4D 3S 2H".parse().unwrap();
    assert_eq!(
      hand.counts(),
      (
        [
          (1, Face::Ace),
          (1, Face::Five),
          (1, Face::Four),
          (1, Face::Three),
          (1, Face::Two),
        ],
        [
          (2, Suit::Hearts),
          (1, Suit::Spades),
          (1, Suit::Diamonds),
          (1, Suit::Clubs),
        ],
        true
      ),
    );
  }
  #[test]
  fn test_display() {
    let hand: Hand = "AH AD AC KS QH".parse().unwrap();
    assert_eq!(hand.to_string(), "AH AD AC KS QH");
  }
}
