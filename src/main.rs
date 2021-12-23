use geng::prelude::*;

mod constants;
mod diagram;
mod draw;
mod editor_state;
mod game_state;

fn main() {
    logger::init().unwrap();
    geng::setup_panic_handler();

    // Setup working directory
    if let Some(dir) = std::env::var_os("CARGO_MANIFEST_DIR") {
        std::env::set_current_dir(std::path::Path::new(&dir).join("static")).unwrap();
    } else {
        #[cfg(not(target_arch = "wasm32"))]
        {
            if let Some(path) = std::env::current_exe().unwrap().parent() {
                std::env::set_current_dir(path).unwrap();
            }
        }
    }

    let geng = Geng::new("Anlaut Winter Jam 2021");
    let state = game_state::GameState::new(&geng, Some(constants::DIAGRAM_FILE));

    geng::run(&geng, state);
}
