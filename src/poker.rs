// Student name: Saba Zubair, 500711970
// Partner: Nick Metzler, 501050712
// Course: CCPS506, Prof Ufkes
// Date: Dec 11, 2021

// To run: type `cargo run` in your terminal.

use crate::Deck;

pub fn deal(perm: [u32; 9]) -> Vec<String> {
  // create a deck from perm values
  let deck = Deck::new(perm);
  // deal out two players
  let [p1, p2] = deck.deal();
  // determine winning hand from each player, use best_hand
  let best_hand_p1 = p1.best_hand();
  let best_hand_p2 = p2.best_hand();
  let winner = best_hand_p1.max(best_hand_p2);
  // return vec as a string, use .as_str_vec()
  winner.as_str_vec()
}
