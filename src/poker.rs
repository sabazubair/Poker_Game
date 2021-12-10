use crate::Deck;
// use crate::Deal; - only needed if you're using Deal.____

pub fn deal(perm: [u32; 9]) -> Vec<String> {
  // create a deck from perm values
  let deck = Deck::new(perm);
  // deal out two players
  let [p1, p2] = deck.deal();

  // let both = deck.deal();
  // let p1 = both[0];
  // let b2 = both[1];

  // determine winning hand from each player, use best_hand
  let best_hand_p1 = p1.best_hand();
  let best_hand_p2 = p2.best_hand(); //getting back two hands
  let winner = best_hand_p1.max(best_hand_p2);
  // compare the two, pick winning hand
  winner.as_str_vec() // return winning hand .to_str_vec()
                      // Vec::new()
}
