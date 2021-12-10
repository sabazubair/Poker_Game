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
    match hand.counts() {
      (
        [(1, Face::Ace), (1, Face::King), (1, Face::Queen), (1, Face::Jack), (1, Face::Ten)],
        [(5, _), _, _, _],
        _,
      ) => Self::RoyalFlush,
      ([(1, a), _, _, _, _], [(5, _), _, _, _], true) => Self::StraightFlush(a),
      ([(4, a), (1, b), _, _, _], _, _) => Self::FourOfAKind(a, [b]),
      ([(3, a), (2, b), _, _, _], _, _) => Self::FullHouse(a, b),
      ([(1, a), (1, b), (1, c), (1, d), (1, e)], [(5, _), _, _, _], _) => {
        Self::Flush([a, b, c, d, e])
      }
      ([(1, a), _, _, _, _], _, true) => Self::Straight(a),
      ([(3, a), (1, b), (1, c), _, _], _, _) => Self::ThreeOfAKind(a, [b, c]),
      ([(2, a), (2, b), (1, c), _, _], _, _) => Self::TwoPair(a, b, [c]),
      ([(2, a), (1, b), (1, c), (1, d), _], _, _) => Self::Pair(a, [b, c, d]),
      _ => Self::HighCard(hand.cards().map(|c| c.face())),
    }
  }
}
// impl Ord
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_royal_flush() {
    let hand: Hand = "AH KH QH JH TH".parse().unwrap();
    let rank: Rank = hand.into();
    assert_eq!(Rank::RoyalFlush, rank);
  }
  #[test]
  fn test_straight_flush() {
    let hand: Hand = "6H 7H 8H 9H TH".parse().unwrap();
    let rank: Rank = hand.into();
    assert_eq!(Rank::StraightFlush(Face::Ten), rank);
  }
  #[test]
  fn test_four_of_a_kind() {
    let hand: Hand = "AH AC AD AS KH".parse().unwrap();
    let rank: Rank = hand.into();
    assert_eq!(Rank::FourOfAKind(Face::Ace, [Face::King]), rank);
  }
  #[test]
  fn test_full_house() {
    let hand: Hand = "AH AC AD KS KH".parse().unwrap();
    let rank: Rank = hand.into();
    assert_eq!(Rank::FullHouse(Face::Ace, Face::King), rank);
  }
  #[test]
  fn test_flush() {
    let hand: Hand = "KC TC 8C 7C 5C".parse().unwrap();
    let rank: Rank = hand.into();
    assert_eq!(
      Rank::Flush([Face::King, Face::Ten, Face::Eight, Face::Seven, Face::Five]),
      rank
    );
  }
  #[test]
  fn test_straight() {
    let hand: Hand = "TH 9C 8D 7S 6H".parse().unwrap();
    let rank: Rank = hand.into();
    assert_eq!(Rank::Straight(Face::Ten), rank);
  }
  #[test]
  fn test_threeofakind() {
    let hand: Hand = "AH AD AC KS QH".parse().unwrap();
    let rank: Rank = hand.into();
    assert_eq!(
      Rank::ThreeOfAKind(Face::Ace, [Face::King, Face::Queen]),
      rank
    );
  }
  #[test]
  fn test_twopair() {
    let hand: Hand = "AH AD KD KS 7H".parse().unwrap();
    let rank: Rank = hand.into();
    assert_eq!(Rank::TwoPair(Face::Ace, Face::King, [Face::Seven]), rank);
  }
  #[test]
  fn test_pair() {
    let hand: Hand = "AH AD KD JS 7H".parse().unwrap();
    let rank: Rank = hand.into();
    assert_eq!(
      Rank::Pair(Face::Ace, [Face::King, Face::Jack, Face::Seven]),
      rank
    );
  }
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
