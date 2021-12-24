use geng::prelude::*;

mod assets;
mod constants;
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

    geng::run(&geng, loading_screen(&geng));
}

fn loading_screen(geng: &Geng) -> impl geng::State {
    let assets = <assets::Assets as geng::LoadAsset>::load(&geng, ".");
    geng::LoadingScreen::new(&geng, geng::EmptyLoadingScreen, assets, {
        let geng = geng.clone();
        move |assets| {
            let mut assets = assets.unwrap();
            assets.init();

            game_state::GameState::new(&geng, &Rc::new(assets))
        }
    })
}
