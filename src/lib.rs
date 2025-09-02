
use wasm_bindgen::prelude::*;

// A simple fruit struct (only one fruit for now)
#[wasm_bindgen]
pub struct Fruit {
    x: f32,
    y: f32,
}

#[wasm_bindgen]
impl Fruit {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f32, y: f32) -> Fruit {
        Fruit { x, y }
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }
}

#[wasm_bindgen]
pub struct Game {
    fruit: Fruit,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Game {
        Game {
            fruit: Fruit::new(100.0, 100.0),
        }
    }

    pub fn tick(&mut self) {
        // For now, just move fruit slightly
        self.fruit.y += 1.0;
    }

    pub fn get_fruit(&self) -> Fruit {
        Fruit::new(self.fruit.x, self.fruit.y)
    }

    pub fn spawn_fruit(&mut self, x: f32, y: f32) {
        self.fruit = Fruit::new(x, y);
    }
}
