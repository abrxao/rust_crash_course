#![deny(clippy::all)]
use intutils::addition::add;
use intutils::subtraction::subtract;
fn main() {
    let a: i32 = 1;
    let b: i32 = 2;
    println!("subtracting {}", subtract(a, b));
    println!("adding {}", add(a, b));
}
