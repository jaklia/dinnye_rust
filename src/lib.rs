// include & re-export the wasm wrapper
mod wasm_interface;
pub use wasm_interface::WasmGame;

//use wasm_bindgen::prelude::*;
use std::ops;

trait Sim { 
    fn step(&mut self, gravity: f32) {}
}

//  Point
#[derive(Debug,Copy,Clone)]
struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn new(x: f32, y: f32) -> Point {
        Point { x, y }
    } 
}

impl ops::Add<Point> for Point {
    type Output = Point;
    fn add(self, _rhs: Point) -> Point {
        Point { x: self.x + _rhs.x, y: self.y + _rhs.y }
    }
}

impl ops::AddAssign<Point> for Point {
    fn add_assign(&mut self, _rhs: Point) -> () {
        self.x += _rhs.x;
        self.y += _rhs.y;
      //  ()
    }
}

impl ops::Sub<Point> for Point {
    type Output = Point;
    fn sub(self, _rhs: Point) -> Point {
        Point { x: self.x - _rhs.x, y: self.y - _rhs.y }
    }
}

impl ops::SubAssign<Point> for Point {
    fn sub_assign(&mut self, _rhs: Point) -> () {
        self.x -= _rhs.x;
        self.y -= _rhs.y;
      //  ()
    }
}

//   /Point  


// A simple fruit struct (only one fruit for now)
//#[wasm_bindgen]
pub struct Fruit {
    //x: f32,
    //y: f32,
    pos: Point,
    last_pos: Point,
}

//#[wasm_bindgen]
impl Fruit {
   // #[wasm_bindgen(constructor)]
    pub fn new(x: f32, y: f32) -> Fruit {
        Fruit { 
            pos: Point { x, y },
            last_pos: Point { x, y },
         }
    }

    // pub fn x(&self) -> f32 {
    //     self.x
    // }

    // pub fn y(&self) -> f32 {
    //     self.y
    // }  
}

impl Sim for Fruit {
    fn step(&mut self, gravity: f32) {
        let v = self.pos - self.last_pos;
        self.last_pos = self.pos;
        self.pos += v + Point { x: 0., y: gravity };
    }
}

//#[wasm_bindgen]
pub struct Game {
   // fruit: Fruit,
   pub fruits: Vec::<Fruit>,
}

//#[wasm_bindgen]
impl Game {
   // #[wasm_bindgen(constructor)]
    pub fn new() -> Game {
        Game {
            fruits: vec! [
                Fruit::new(100.0, 100.0),
                Fruit::new(50., 150.),
                 ],
        }
    }

    pub fn tick(&mut self) {
        // For now, just move fruit slightly
       // self.fruit.y += 1.0;

        Game::step(self, 0.08);
    }

    // pub fn get_fruit(&self) -> Fruit {
    //     Fruit::new(self.fruit.x, self.fruit.y)
    // }

    pub fn spawn_fruit(&mut self, x: f32, y: f32) {
        self.fruits.push(Fruit::new(x, y));
    }
}

impl Sim for Game {
    fn step(&mut self, gravity: f32) {
       
        self.fruits.iter_mut().for_each(|fruit| {
            fruit.step(gravity);
        });

      self.fruits.retain(|fruit|  {fruit.pos.y < 500.});

        // for mut fruit in &self.fruits {
        //     fruit.step(gravity);
        // }
    }
}


