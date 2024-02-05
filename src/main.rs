#![deny(clippy::all)]
use std::cell::Cell;
struct Person {
    name: String,
    age: Cell<u8>,
}
impl Person {
    fn increment_age(&self) {
        self.age.set(self.age.get() + 1);
    }
}
fn main() {
    let person = Person {
        name: "John".to_string(),
        age: Cell::new(12),
    };
    person.increment_age();
    println!("{}", person.age.get())
}
