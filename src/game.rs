use rand::{rng, seq::SliceRandom};
use uuid::Uuid;
use std::{error::Error, fmt::Display};

use crate::card_set::shuffle_deck;

pub enum DrawAction {
    Draw,
    Discard,
}

#[derive(Debug)]
pub struct GameError;
impl Error for GameError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "uh"
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}
impl Display for GameError {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

pub struct Player {
    id: String,
    pub name: String,
    cards: (Vec<i8>, u8),
}
impl Player {
    pub fn init(name: String) -> Self {
        Self { id: String::new(), name, cards: (vec![-1, -1, -1, -1, -1, -1, -1, -1], 0) }
    }
    /// Values of -1 mean it is face down.
    pub fn face_up_cards(&self) -> Vec<i8> {
        let mut ret = self.cards.0.clone();

        for i in 0..8u8 {
            if self.cards.1 & (0b1 << i) == 0 {
                ret[0] = -1;
            }
        }

        ret
    }
}

/// Player count requirements: min 2, max 6.
pub struct Game {
    id_list: Vec<String>,
    pub turn: usize,  // 6 indicates the round hasn't started or it has ended
    pub round: usize, // 0 indicates the game hasn't started; 11 indicates the game has ended
    pub draw_pile: Vec<i8>,
    pub discard_pile: Vec<i8>,
}
impl Game {
    /// Shuffles the deck, deals the cards out, and prepares the decks.
    pub fn init_game(players: &mut [Player]) -> Self {
        let mut draw_pile = shuffle_deck();
        
        let mut id_list = Vec::new();
        for player in &mut *players {
            let id = Uuid::new_v4().to_string();
            player.id = id.clone();
            id_list.push(id);
        }
        for _ in 0..8 {
            for player in &mut *players {
                player.cards.0.push(draw_pile.pop().unwrap());
            }
        }

        

        Self {
            id_list,
            turn: 0,
            round: 0,
            draw_pile,
            discard_pile: Vec::new(),
        }
    }

    /// Right before the game starts, everyone turns up 2 of their cards of their choice.
    pub fn start_game(&mut self, turn_up_cards: Vec<&[usize; 2]>) -> Result<(), GameError> {
        if turn_up_cards.len() != self.id_list.len() {
            eprintln!(
                "GameError: given vector for turning cards up did not equal the amount of players."
            );
            return Err(GameError);
        }

        for i in 0..turn_up_cards.len() {
            if turn_up_cards[i][0] >= 8 || turn_up_cards[i][i] >= 8 {
                eprintln!("GameError: index card value greater than 7.");
                return Err(GameError);
            } else if self.id_list[i].cards.1 != 0 {
                eprintln!(
                    "GameError: the cards have already been chosen for this player, skipping player."
                );
                continue;
            }
            self.id_list[i].cards.1 = 1 << turn_up_cards[i][0] | 1 << turn_up_cards[i][1];
        }

        self.turn = 0;

        Ok(())
    }

    pub fn draw_card(&mut self, from: DrawAction) -> Result<i8, GameError> {
        match from {
            DrawAction::Draw => match self.draw_pile.pop() {
                Some(v) => Ok(v),
                None => {
                    eprintln!("GameError: no cards in the draw pile (WIP)");
                    Err(GameError)
                }
            },
            DrawAction::Discard => match self.discard_pile.pop() {
                Some(v) => Ok(v),
                None => {
                    eprintln!("GameError: no cards in the discard pile (WIP)");
                    Err(GameError)
                }
            },
        }
    }
}
