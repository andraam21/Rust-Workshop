pub fn run() {
    // TODO 1 - fix the sum
    let a: u32 = 32;
    let b: i32 = 32;
    println!("the sum is {}", a as i32 + b);

    // TODO 2 - fix the printing
    let t = 'n';
    println!("the next character after {} is {}", t, std::char::from_u32(t as u32 + 1).unwrap_or(t));
}