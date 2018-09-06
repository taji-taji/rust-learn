fn main() {
    // Integer Types
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("guess: {}", guess);

    // Floating-Point Types
    let x = 2.0; // default f64
    println!("x: {}", x);

    let y: f32 = 3.0;
    println!("y: {}", y);

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

    // Boolean Types
    let t = true;
    println!("t: {}", t);

    let f: bool = false;
    println!("f: {}", f);

    // Character Types
    let c = 'z';
    println!("c: {}", c);
    
    let z = 'ℤ';
    println!("z: {}", z);

    let heart_eyed_cat = '😻';
    println!("heart_eyed_cat: {}", heart_eyed_cat);

    // Tuple Type
    let _tup: (i32, f64, u8) = (500, 6.4, 1);

    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}