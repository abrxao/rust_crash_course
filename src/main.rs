#![deny(clippy::all)]

struct Person<'a>{
    first_name: &'a str,
    last_name: &'a str,
}

impl<'a> Person<'a> {
    fn fisrt_char_of_first_name(&self) -> &str{
        &self.first_name[0..1]
    }
    fn get_full_name(&self) -> String{
        format!("{} {}", self.first_name, self.last_name)
    }
}

enum Animal<'a>{
    Dog{name: &'a str}
}

fn main() {
    let person = Person{ first_name: "John", last_name: "Doe"};
    let full_name = person.get_full_name();
    println!("{:?}",full_name );
}
/*
#[cfg(test)]
mod tests {} */
