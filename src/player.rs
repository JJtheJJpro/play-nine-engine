pub struct Players {
    list: Vec<Player>,
    turn: usize,
}

pub struct Player {
    name: String,
    cards: (Vec<i8>, u8),
}
impl Player {
    pub fn name(&self) -> String {
        self.name.clone()
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