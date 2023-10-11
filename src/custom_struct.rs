pub trait PlanetAction {
    fn code(&self) -> usize;
}

#[derive(Debug)]
pub struct Planet;

impl PlanetAction for Planet {
    fn code(&self) -> usize {
        println!("Alpha 001T");
        1
    }
}

// Blanket implementation
impl<T: PlanetAction> PlanetAction for &T {
    fn code(&self) -> usize {
        (*self).code() + 10
    }
}
