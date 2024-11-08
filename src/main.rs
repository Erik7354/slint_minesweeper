mod minesweeper;
mod ui;

const SETTINGS: minesweeper::Settings = minesweeper::BEGINNER_SETTINGS;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen(start))]
pub fn main() {
    let ms = minesweeper::new(SETTINGS);

    ui::run(ms).unwrap();
}

