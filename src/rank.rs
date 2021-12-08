use crate::Face;
use crate::Hand;

#[derive(Clone, Debug, Copy, Eq, PartialEq)]
pub enum Rank {
  RoyalFlush,
  StraightFlush(Face),
  FourOfAKind(Face, [Face; 1]),
  FullHouse(Face, Face),
  Flush([Face; 5]),
  Straight(Face),
  ThreeOfAKind(Face, [Face; 2]),
  TwoPair(Face, Face, [Face; 1]),
  Pair(Face, [Face; 3]),
  HighCard([Face; 5]),
}

impl From<Hand> for Rank {
  fn from(hand: Hand) -> Self {
    Self::HighCard(hand.cards().map(|c| c.face()))
  }
}
// impl Ord
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_high_card() {
    let hand: Hand = "AH KS QD 9S 7H".parse().unwrap();
    let rank: Rank = hand.into();
    assert_eq!(
      Rank::HighCard([Face::Ace, Face::King, Face::Queen, Face::Nine, Face::Seven]),
      rank
    );
  }
}
