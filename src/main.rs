use std::ops::Index;

mod minesweeper;
mod ui;

#[cfg(not(target_arch = "wasm32"))]
pub fn main() {
    let settings: minesweeper::Settings = minesweeper::BEGINNER_SETTINGS;

    let ms = minesweeper::new(settings);
    ui::run(ms).unwrap();
}

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[cfg(target_arch = "wasm32")]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen(start))]
pub fn main() {
    log!("Welcome to Minesweeper made with Rust+Slint and running on WASM!");

    let mut settings: minesweeper::Settings = minesweeper::BEGINNER_SETTINGS;

    let win = web_sys::window().unwrap();
    let s = win.location().search().unwrap();

    let query = s.trim_start_matches("?");
    if query != "" {
        for entry in query.split("&").collect::<Vec<_>>() {
            let pair: Vec<&str> = entry.split("=").collect();
            let key = pair[0];
            let val = pair[1];

            match key {
                "width" => settings.dx = val.parse::<usize>().unwrap(),
                "height" => settings.dy = val.parse::<usize>().unwrap(),
                "mine_count" => settings.mine_count = val.parse::<usize>().unwrap(),
                _ => ()
            }
        }
    }

    let ms = minesweeper::new(settings);
    ui::run(ms).unwrap();
}

