fn main() {
    // Integer Types
    let _guess: u32 = "42".parse().expect("Not a number!");

    // Floating-Point Types
    let _x = 2.0; // default f64

    let _y: f32 = 3.0;

    // Numeric operation
    let sum = 5 + 10;
    println!("sum: {}", sum);

    let difference = 95.5 - 4.3;
    println!("difference: {}", difference);

    let product = 4 * 30;
    println!("product: {}", product);

    let quotient = 56.7 / 32.2;
    println!("quotient: {}", quotient);

    let remainder = 43 % 5;
    println!("remainder: {}", remainder);
}