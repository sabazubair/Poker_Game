use crate::Card;
use crate::Deal;

#[derive(Clone, Debug, Copy, Eq, PartialEq)]
pub struct Deck {
  cards: [Card; 9],
}
impl Deck {
  pub fn new(cards: [u32; 9]) -> Self {
    Self {
      cards: cards.map(|c| c.into()),
    }
  }
  pub fn deal(&self) -> [Deal; 2] {
    let community = self.cards[4..9].try_into().unwrap(); // try_into converts arr slice into an arr

    [
      Deal::new([self.cards[0], self.cards[2]], community),
      Deal::new([self.cards[1], self.cards[3]], community),
    ]
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_deal() {
    let deck = Deck::new([40, 41, 42, 43, 48, 49, 50, 51, 52]);
    let deal = deck.deal();
    assert_eq!(deal[0].to_string(), "AS 3S + 9S TS JS QS KS");
    assert_eq!(deal[1].to_string(), "2S 4S + 9S TS JS QS KS");
  }
}
