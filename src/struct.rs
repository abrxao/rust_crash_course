#![deny(clippy::all)]
use rand::Rng;
use std::f64::consts::PI;

#[derive(Debug)]
struct Coordinates(f64, f64, f64);

impl Coordinates {
    fn get_distance_from(&self, point: &Coordinates) -> f64 {
        let distance = (self.0 - point.0).powf(2.0)
            + (self.1 - point.1).powf(2.0)
            + (self.2 - point.2).powf(2.0);
        distance.abs().sqrt()
    }
    fn origin() -> Coordinates {
        Coordinates(0.0, 0.0, 0.0)
    }
}
fn main() {
    let origin = Coordinates::origin();
    let phy = PI / 4.0;
    let omega: f64 = PI / 4.0;
    let mut rng = rand::thread_rng();
    let radius: f64 = rng.gen::<f64>() * 20.0;
    let z: f64 = radius * phy.cos();
    let x = radius * omega.cos() * phy.sin();
    let y = radius * omega.sin() * phy.sin();

    let point = Coordinates(x, y, z);
    println!(
        "Radius is {}\nCalculate distance is {}",
        radius,
        origin.get_distance_from(&point)
    );
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_distance_from() {
        let origin = Coordinates(0.0, 0.0, 0.0);
        let phy = PI / 3.0;
        let omega: f64 = PI / 3.0;
        let mut rng = rand::thread_rng();
        let radius: f64 = rng.gen::<f64>() * 20.0;
        let z: f64 = radius * phy.cos();
        let x = radius * omega.cos() * phy.sin();
        let y = radius * omega.sin() * phy.sin();

        let point = Coordinates(x, y, z);
        println!(
            "Radius is {}\nCalculate distance is {}",
            radius,
            origin.get_distance_from(&point)
        );
        assert!((origin.get_distance_from(&point) - radius).abs() < 0.0000000001);
    }
}
