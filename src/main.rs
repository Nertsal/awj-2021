use geng::prelude::*;

mod game_state;
mod diagram;
mod constants;

fn main() {
    logger::init().unwrap();
    geng::setup_panic_handler();

    let geng = Geng::new("Anlaut Winter Jam 2021");
    let state = game_state::GameState::new(&geng);

    geng::run(&geng, state);
}
