use rust_play::custom_struct::{Planet, PlanetAction};

fn speak_to<T>(s: T) -> usize
where
    T: PlanetAction,
{
    s.code()
}

fn main() {
    let planet: Planet = Planet;
    println!("PlanetX {:?}", planet);
    println!("Value {:?}", speak_to(planet));
    let planet2: Planet = Planet;
    println!("Value {:?}", speak_to(&planet2));
}
