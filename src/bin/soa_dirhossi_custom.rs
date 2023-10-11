use rand::Rng;
use std::time::Instant;

const N: usize = 1_000_000;
const N_WARMUP: usize = 100;

struct Shapes {
    rectangles: Vec<Rectangle>,
    triangles: Vec<Triangle>,
    squares: Vec<Square>,
}

impl Shapes {
    fn total_area(&self) -> f32 {
        self.rectangles
            .iter()
            .map(|r| r.area())
            .chain(self.triangles.iter().map(|t| t.area()))
            .chain(self.squares.iter().map(|s| s.area()))
            .sum()
    }
}

trait Calculation{
    fn area(&self) -> f32;
}


struct Square {
    side: f32,
}
impl Calculation for Square{
    #[inline(always)]
    fn area(&self) -> f32 {
        self.side * self.side
    }
}

struct Rectangle {
    width: f32,
    height: f32,
}
impl Calculation for Rectangle {
    #[inline(always)]
    fn area(&self) -> f32 {
        self.width * self.height
    }
}

struct Triangle {
    base: f32,
    height: f32,
}
impl Calculation for Triangle {
    #[inline(always)]
    fn area(&self) -> f32 {
        0.5 * self.base * self.height
    }
}
fn init_shapes() -> Shapes {
    let mut rectangles: Vec<Rectangle> = Vec::with_capacity(N);
    let mut triangles: Vec<Triangle> = Vec::with_capacity(N);
    let mut squares: Vec<Square> = Vec::with_capacity(N);
    let mut rng = rand::thread_rng();
    for _ in 0..3 * N {
        match rng.gen_range(0..3) {
            0 => {
                rectangles.push(Rectangle {
                    width: 4.0,
                    height: 4.0,
                });
            }
            1 => triangles.push(Triangle {
                base: 4.0,
                height: 4.0,
            }),
            _ => squares.push(Square { side: 4.0 }),
        }
    }
    Shapes {
        rectangles,
        triangles,
        squares,
    }
}
fn main() {
    let shapes = init_shapes();

    // Running benchmark
    let start = Instant::now();
    let mut total = 0.0;
    for _ in 0..N_WARMUP {
        total = shapes.total_area();
    }
    let duration = start.elapsed();

    println!(
        "{}. Struct-of-arrays Trait took {:?}.",
        total,
        duration / N_WARMUP as u32
    );
}