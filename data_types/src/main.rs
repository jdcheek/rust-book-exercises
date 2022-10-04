fn main() {
    // floating point types
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

    //numeric operations
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0

    // remainder
    let remainder = 43 % 5;

    // bool
    let t = true;

    let f: bool = false; // with explicit type annotation
    
    // character type 
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation

    // compound types
    // tuple
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    // arrays
    let a: [i32; 5] = [1, 2, 3, 4, 5]; // type and size written in brackets
    let first = a[0]; // set variables to array values at index
    let second = a[1];
    let a = [3; 5]; // initialize array with 5 elements with value of int 3
}
