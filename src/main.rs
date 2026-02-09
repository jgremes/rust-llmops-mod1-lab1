use calc_lib::{add, divide, multiply, power, subtract};

fn main() {
    let a = 12;
    let b = 6;
    let r = 20;

    let
    
    let sum = add(a, b);
    let difference = subtract(a, b);
    let product = multiply(a, b);
    let quotient = divide(a, b);
    let power = power(a, b);

    println!("Sum: {}", sum);
    println!("Difference: {}", difference);
    println!("Product: {}", product);
    println!("Quotient: {}", quotient);
    println!("Power: {}", power);
}
