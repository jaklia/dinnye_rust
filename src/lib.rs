// include & re-export the wasm wrapper
mod wasm_interface;
pub use wasm_interface::WasmGame;

//use wasm_bindgen::prelude::*;
use std::ops;

trait Sim {
    fn step(&mut self, gravity: f32) {}
}

//  Point
#[derive(Debug, Copy, Clone)]
struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn new(x: f32, y: f32) -> Point {
        Point { x, y }
    }
    fn dist(&self, other: &Point) -> f32 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

impl ops::Add<Point> for Point {
    type Output = Point;
    fn add(self, _rhs: Point) -> Point {
        Point {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
        }
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
        Point {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
        }
    }
}

impl ops::SubAssign<Point> for Point {
    fn sub_assign(&mut self, _rhs: Point) -> () {
        self.x -= _rhs.x;
        self.y -= _rhs.y;
        //  ()
    }
}

impl ops::Mul<f32> for Point {
    type Output = Point;
    fn mul(self, _rhs: f32) -> Point {
        Point {
            x: self.x * _rhs,
            y: self.y * _rhs,
        }
    }
}

impl ops::MulAssign<f32> for Point {
    fn mul_assign(&mut self, _rhs: f32) -> () {
        self.x *= _rhs;
        self.y *= _rhs;
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
    rot: f32,
    r: f32,
    i: i8,
}

//#[wasm_bindgen]
impl Fruit {
    // #[wasm_bindgen(constructor)]
    pub fn new(x: f32, y: f32, rot: f32, i: i8) -> Fruit {
        Fruit {
            pos: Point { x, y },
            last_pos: Point { x, y },
            rot: rot,
            i: 0,
            r: (i as f32 + 1.0).powf(1.234) * 3.2 * 1.414,
        }
    }

    fn constrain(&mut self, max_x: f32, max_y: f32) {
        if self.pos.x < self.r {
            let d_x = self.pos.x - self.last_pos.x;
            self.pos.x = self.r;
           self.last_pos.x = self.pos.x + d_x*0.6;
        }
        if self.pos.x > max_x - self.r {
            let d_x = self.pos.x - self.last_pos.x;
            self.pos.x = max_x - self.r;
           self.last_pos.x = self.pos.x + d_x*0.6;
        }
        if self.pos.y > max_y - self.r {
            let d_y = self.pos.y - self.last_pos.y;
            self.pos.y = max_y - self.r;
            self.last_pos.y = self.pos.y  /*+ d_y*/;
        }
    }

    fn resolve(&mut self, other: &mut Fruit) {
        let rsum = self.r + other.r; 
        let dist = self.pos.dist(&other.pos);
        if dist <= rsum { 
            // diff: how much objs overlap
            let diff = (rsum - dist) * 0.95;

            // how much to move each obj, 
            //   /dist so the (pos1 - pos2) vec is normalized (later when we multiply it)
            let n = diff /  dist;
            //   it could be (* .5) at the end, but: we want the smaller obj to 
            let a1 =  n * other.r / rsum;
            let a2 =  n * self.r / rsum;

            let d1 = (self.pos - other.pos) * a1;
            let d2 = (other.pos - self.pos) * a2;

            self.pos += d1;
            other.pos += d2;
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
        let v = (self.pos - self.last_pos) ;

        self.last_pos = self.pos;
        self.pos.y += gravity;
        self.pos += v;
    }
}

//#[wasm_bindgen]
pub struct Game {
    // fruit: Fruit,
    pub fruits: Vec<Fruit>,
    currentFruit: Fruit,
    nextFruit: Fruit,
}

//#[wasm_bindgen]
impl Game {
    // #[wasm_bindgen(constructor)]
    pub fn new() -> Game {
        Game {
            currentFruit: Fruit::new(0., 0., 0., 0),
            nextFruit: Fruit::new(0., 0., 0., 0),
            fruits: vec![
                Fruit::new(100.0, 100.0, 0., 0),
                Fruit::new(50., 150., 0., 0),
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
        self.fruits.push(Fruit::new(x, y, 0., 1));
    }
}

impl Sim for Game {
    fn step(&mut self, gravity: f32) {
        self.fruits.iter_mut().for_each(|fruit| {
            fruit.step(gravity);
        });

        // for mut f1 in &self.fruits {
        //     for mut f2 in &self.fruits {
        //         f1.resolve(&mut f2);
        //     }
        // }

        for _ in 0..8 {
            let n = self.fruits.len();
            for i in 0..n {
                if i < n - 1 {
                    let smthing = self.fruits.split_at_mut(i + 1);

                    for j in i + 1..n {
                        if i != j {
                            //    smthing.0[i].resolve(&mut smthing.1[j-i]);
                            smthing.0[i].resolve(&mut smthing.1[j - i - 1]);
                        }
                    }
                }
            }

            self.fruits.iter_mut().for_each(|fruit| {
                fruit.constrain(400., 400.);
            });
        }

        //  self.fruits.retain(|fruit|  {fruit.pos.y < 500.});

        // for mut fruit in &self.fruits {
        //     fruit.step(gravity);
        // }
    }
}
