#![deny(clippy::all)]

fn main() {
    let number: Option<i32> = Some(12);
    let mult_by_2 = number.map_or(20, |value| value * 2);
    println!("{}", mult_by_2);
    match number {
        Some(number) => println!("{}", number),
        None => println!("No number yet"),
    }
}
