use rand::{rng, seq::SliceRandom};

/**
 * There are 8 cards of each number except for the four -5's.
 * Except for the -5's, the card numbers range from 0-12.
 * 8 * 13 + 4 = 108
 * There are 108 cards total.
 */
const DECK: &[i8; 108] = &[
    -5, -5, -5, -5, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 3, 3,
    3, 3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 4, 4, 4, 5, 5, 5, 5, 5, 5, 5, 5, 6, 6, 6, 6, 6, 6, 6, 6, 7, 7,
    7, 7, 7, 7, 7, 7, 8, 8, 8, 8, 8, 8, 8, 8, 9, 9, 9, 9, 9, 9, 9, 9, 10, 10, 10, 10, 10, 10, 10,
    10, 11, 11, 11, 11, 11, 11, 11, 11, 12, 12, 12, 12, 12, 12, 12, 12,
];

/**
 * Layout of cards (by index):
 * 0  1  2  3
 * 4  5  6  7
 * In the player struct, the 2nd tuple member of the cards field indicates by flags which cards are up (0 if down, 1 if up)
 * by index (0b00010111 indicates 0, 1, 2, and 4 are up and the rest are down.)
 */
pub fn shuffle_deck() -> Vec<i8> {
    let mut ret = DECK.to_vec();
    ret.shuffle(&mut rng());
    ret
}
