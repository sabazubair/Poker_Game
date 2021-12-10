use crate::Card;
use crate::Hand;
use std::fmt;

fn combinations(arr: &[Card], size: usize) -> Vec<Vec<Card>> {
  if size == 0 {
    return vec![Vec::new()];
  }
  if size > arr.len() {
    return Vec::new();
  }
  let mut result = Vec::new();
  let first_elem = arr[0];
  let combos = combinations(&arr[1..], size - 1);

  for combo in combos {
    let one_combo = [&[first_elem], combo.as_slice()].concat();
    result.push(one_combo);
  }

  let combos = combinations(&arr[1..], size);
  for combo in combos {
    result.push(combo);
  }
  result
}

#[derive(Clone, Debug, Copy, Eq, PartialEq)]
pub struct Deal {
  hole: [Card; 2],
  community: [Card; 5],
}

impl Deal {
  pub fn new(hole: [Card; 2], community: [Card; 5]) -> Self {
    Self { hole, community }
  }
  pub fn combine(&self) -> [Card; 7] {
    [
      self.hole[0],
      self.hole[1],
      self.community[0],
      self.community[1],
      self.community[2],
      self.community[3],
      self.community[4],
    ]
  }
  pub fn combinations(&self) -> Vec<Hand> {
    combinations(&self.combine(), 5)
      .into_iter()
      .map(|v| Hand::new(v.as_slice()))
      .collect()
  }
  pub fn best_hand(&self) -> Hand {
    let mut hands = self.combinations();
    hands.sort();
    hands.pop().unwrap()
  }
}

impl fmt::Display for Deal {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "{} {} + {} {} {} {} {}",
      self.hole[0],
      self.hole[1],
      self.community[0],
      self.community[1],
      self.community[2],
      self.community[3],
      self.community[4],
    )
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_to_string() {
    let deal = Deal::new(
      ["AS", "3S"].map(|c| c.parse().unwrap()),
      ["9S", "TS", "JS", "QS", "KS"].map(|c| c.parse().unwrap()),
    );
    assert_eq!(deal.to_string(), "AS 3S + 9S TS JS QS KS");
  }
  #[test]
  fn test_combinations() {
    let deal = Deal::new(
      ["AS", "3S"].map(|c| c.parse().unwrap()),
      ["9S", "TS", "JS", "QS", "KS"].map(|c| c.parse().unwrap()),
    );
    assert_eq!(
      deal
        .combinations()
        .into_iter()
        .map(|h| h.to_string())
        .collect::<Vec<String>>(),
      vec![
        "AS JS TS 9S 3S",
        "AS QS TS 9S 3S",
        "AS KS TS 9S 3S",
        "AS QS JS 9S 3S",
        "AS KS JS 9S 3S",
        "AS KS QS 9S 3S",
        "AS QS JS TS 3S",
        "AS KS JS TS 3S",
        "AS KS QS TS 3S",
        "AS KS QS JS 3S",
        "AS QS JS TS 9S",
        "AS KS JS TS 9S",
        "AS KS QS TS 9S",
        "AS KS QS JS 9S",
        "AS KS QS JS TS",
        "QS JS TS 9S 3S",
        "KS JS TS 9S 3S",
        "KS QS TS 9S 3S",
        "KS QS JS 9S 3S",
        "KS QS JS TS 3S",
        "KS QS JS TS 9S"
      ]
    );
  }
}
