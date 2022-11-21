mod example_scene;

use wasm_bindgen::prelude::wasm_bindgen;
use flask::{game_context::GameContext, palette};

use example_scene::ExampleScene;


pub const WINDOW_WIDTH: u32 = 128;
pub const WINDOW_HEIGHT: u32 = 128;
pub const FULLSCREEN: bool = false;

#[wasm_bindgen(start)]
pub fn start() {
    let scene = Box::new(ExampleScene::new());

    if let Err(error) = GameContext::run(WINDOW_WIDTH, WINDOW_HEIGHT, FULLSCREEN, palette::flask_default(), scene) {
        flask::log(format!("Flask error:\n{}", error).as_str());
    };
}
