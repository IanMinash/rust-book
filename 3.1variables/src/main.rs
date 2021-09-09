fn main() {
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);

    // Constants
    const Y: u32 = 1_000_000;
    println!("The value of y is {}", Y);

    // Shadowing
    let z = 5;
    println!("The value of z is {}", z);
    let z = z + 10;
    println!("The value of z is {}", z);
}
