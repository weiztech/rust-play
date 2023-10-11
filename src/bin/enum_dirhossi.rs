use rand::Rng;
use std::time::Instant;

const N: usize = 1_000_000;
const N_WARMUP: usize = 100;
enum Shape {
    Rectangle(Rectangle),
    Triangle(Triangle),
    Square(Square),
}
impl Shape {
    fn area(&self) -> f32 {
        match self {
            Shape::Rectangle(r) => r.area(),
            Shape::Triangle(t) => t.area(),
            Shape::Square(s) => s.area(),
        }
    }
}
struct Square {
    side: f32,
}
impl Square {
    #[inline(always)]
    fn area(&self) -> f32 {
        self.side * self.side
    }
}

struct Rectangle {
    width: f32,
    height: f32,
}
impl Rectangle {
    #[inline(always)]
    fn area(&self) -> f32 {
        self.width * self.height
    }
}
struct Triangle {
    base: f32,
    height: f32,
}
impl Triangle {
    #[inline(always)]
    fn area(&self) -> f32 {
        0.5 * self.base * self.height
    }
}
fn total_area(shapes: &[Shape]) -> f32 {
    shapes.iter().fold(0.0, |a, s| a + s.area())
}

fn init_shapes() -> Vec<Shape> {
    let mut shapes: Vec<Shape> = Vec::with_capacity(3 * N as usize);
    for _ in 0..3 * N {
        let mut rng = rand::thread_rng();

        match rng.gen_range(0..3) {
            0 => {
                shapes.push(Shape::Rectangle(Rectangle {
                    width: 4.0,
                    height: 4.0,
                }));
            }
            1 => {
                shapes.push(Shape::Triangle(Triangle {
                    base: 4.0,
                    height: 4.0,
                }));
            }
            _ => {
                shapes.push(Shape::Square(Square { side: 4.0 }));
            }
        }
    }
    shapes
}
fn main() {
    let shapes: Vec<Shape> = init_shapes();

    // Running benchmark
    let start = Instant::now();
    let mut total = 0.0;
    for _ in 0..N_WARMUP {
        total = total_area(&shapes);
    }
    let duration = start.elapsed();

    println!("{}. Enum took {:?}.", total, duration / N_WARMUP as u32);
}
