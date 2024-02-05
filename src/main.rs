#![deny(clippy::all)]

trait CanRun {
    fn run(&self);
}
trait CanWalk {
    fn walk(&self);
}
impl<T: CanRun> CanRun for Vec<T> {
    fn run(&self) {
        for item in self {
            item.run();
        }
    }
}
impl<T: CanWalk> CanWalk for Vec<T> {
    fn walk(&self) {
        for item in self {
            item.walk();
        }
    }
}

struct Person {
    name: String,
}
impl CanRun for Person {
    fn run(&self) {
        println!("{} is running", self.name);
    }
}
impl CanWalk for Person {
    fn walk(&self) {
        println!("{} is walking", self.name);
    }
}

struct Elephant {
    name: String,
}

impl CanWalk for Elephant {
    fn walk(&self) {
        println!("Elephant {} is walking", self.name)
    }
}
fn main() {
    let people = vec![
        Person {
            name: "John".to_string(),
        },
        Person {
            name: "Mary".to_string(),
        },
    ];
    people.run();
    people.walk();
    let elephants = vec![
        Elephant {
            name: "John".to_string(),
        },
        Elephant {
            name: "Mary".to_string(),
        },
    ];
    elephants.walk();
}
