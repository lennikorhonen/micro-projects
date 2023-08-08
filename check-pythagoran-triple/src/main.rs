use std::io;

fn main() {
    println!("Give me the value of a: ");

    let mut input_value = String::new();

    io::stdin()
        .read_line(&mut input_value)
        .expect("Failed to read the value");
    
    let a: u32 = input_value.trim().parse().expect("Input is not a positive integer");

    println!("Give me the value of b: ");

    input_value = String::new();

    io::stdin()
        .read_line(&mut input_value)
        .expect("Failed to read the value");

    let b: u32 = input_value.trim().parse().expect("Input is not a positive integer");

    println!("Give me the value of c: ");

    input_value = String::new();

    io::stdin()
        .read_line(&mut input_value)
        .expect("Failed to read the value");

    let c: u32 = input_value.trim().parse().expect("Input is not a positive integer");

    let is_triple: bool = is_pythagoran_triple(a, b, c);
    println!("Triangle ({}, {}, {}) is a Pythagoran triple: {}", a, b, c, is_triple);
}

fn is_pythagoran_triple(a: u32, b: u32, c: u32) -> bool {
    let calculate_left: u32 = a*a + b*b;
    let calculate_right: u32 = c*c;

    if calculate_left == calculate_right {
        return true
    }
    false
}

