#![deny(clippy::all)]
#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
    age: u8,
}

trait HasFullName
where
    Self: HasName,
{
    fn full_name(&self) -> String;
}
impl<T> HasFullName for T
where
    T: HasName,
{
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name(), self.last_name())
    }
}
trait CanInitializeWithFullName {
    fn new(full_name: &str) -> Self;
}
impl CanInitializeWithFullName for Person {
    fn new(full_name: &str) -> Self {
        let parts: Vec<&str> = full_name.split(' ').collect();
        Person {
            first_name: parts[0].to_string(),
            last_name: parts[1].to_string(),
            age: 30,
        }
    }
}

trait HasName {
    fn first_name(&self) -> &str;
    fn last_name(&self) -> &str;
}

fn print_details<T>(value: &T)
where
    T: HasFullName + CanRun,
{
    println!("{}", value.full_name())
}
trait CanRun {
    fn run(&self);
}
impl CanRun for Person {
    fn run(&self) {
        if self.age > 3 {
            println!("Can run")
        } else {
            println!("Cannot run")
        }
    }
}
impl HasName for Person {
    fn first_name(&self) -> &str {
        &self.first_name
    }
    fn last_name(&self) -> &str {
        &self.last_name
    }
}

fn main() {
    let person = Person {
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        age: 3,
    };
    print_details(&person);
    person.run();
}
