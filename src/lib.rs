mod card;
mod deck;
mod player;
mod game_state;

pub fn run_game() {
    let mut game_state = game_state::GameState::new();
    while !game_state.game_over() {
        game_state.enter_room();
    }
}