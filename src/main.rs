use game::Game;

pub mod card_set;
pub mod game;
pub mod player;

/// All plays are almost random (all face up cards will not be touched at all).
fn play_auto_game_fastest(game: &mut Game) {
    game.start_game(vec![&[0, 0], &[0, 0]]).unwrap();

}

fn main() {
    let mut game = Game::init_game(vec!["josh", "tyson"]).unwrap();
    let players = game.players();
    for player in players.borrow().iter() {
        println!("{}'s cards: {:?}", player.name(), player.face_up_cards());
    }
    println!("discard pile: {:?}", game.discard_pile().clone().into_inner());
    
    play_auto_game_fastest(&mut game);
}
