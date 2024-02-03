#![deny(clippy::all)]

fn main() {
    let mut values = vec![1, 2, 3, 5, 8];
    values.push(13);

    for value in values.iter() {
        let double_val: i32 = 2 * value;
        println!("{}", double_val)
    }
    let m_b2: Vec<i32> = values.into_iter().map(|value| value * 2).collect();
    println!("{:?}", m_b2)
}
