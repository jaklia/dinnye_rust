use wasm_bindgen::prelude::*;
use crate::Game;

#[wasm_bindgen]
pub struct WasmGame {
    game: Game,
}

#[wasm_bindgen]
impl WasmGame {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmGame {
        WasmGame { game: Game::new() }
    }

    pub fn tick(&mut self) {
        self.game.tick();
    }

    pub fn spawn_fruit(&mut self, x: f32, y: f32) {
        self.game.spawn_fruit(x, y);
    }

    /// Returns [x, y, rotation, radius, x, y, rotation, radius, ...]
    pub fn get_fruit_buffer(&self) -> Vec<f32> {
        self.game.fruits
            .iter()
            .flat_map(|f| [f.pos.x, f.pos.y, 0., 20.])
            .collect()
    }

    //  / Returns the fruit state as a JsValue (JSON object)
    // pub fn get_fruit(&self) -> Vec::< {
    //     serde_wasm_bindgen::to_value(&self.game.get_fruit()).unwrap()
    // }


}

