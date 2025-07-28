mod card;
mod deck;
mod player;
mod game_state;

pub fn run_game() {
    let mut game_state = game_state::GameState::new();

    game_state.print_deck();
    game_state.print_player();
    println!("Game started");
    game_state.start_turn();
}