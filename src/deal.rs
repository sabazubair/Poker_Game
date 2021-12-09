use crate::Card;
use std::fmt;

#[derive(Clone, Debug, Copy, Eq, PartialEq)]
pub struct Deal {
  hole: [Card; 2],
  community: [Card; 5],
}

impl Deal {
  pub fn new(hole: [Card; 2], community: [Card; 5]) -> Self {
    Self { hole, community }
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
