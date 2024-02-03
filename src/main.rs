#![deny(clippy::all)]

fn get_user_name() -> Result<String, ()> {
    Ok("hello".to_string())
}

fn get_err_name() -> Result<String, ()> {
    Err(())
}
fn get_double_name() -> Result<String, ()> {
    let name_1 = get_user_name()?;
    let name_2 = get_err_name()?;
    Ok(format!("{} {}", name_1, name_2))
}
fn main() {
    let name = get_double_name();
    match name {
        Ok(name) => println!("testing {}", name),
        Err(_) => println!("Some error ocurred"),
    }
}
