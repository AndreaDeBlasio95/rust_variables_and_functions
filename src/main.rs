fn main() {
    // immutable variable
    let x= 5;
    println!("The value of x is: {}", x);

    // mutable variable
    let mut y = 5;
    println!("The value of y before editing: {}", y);
    y = 6;
    println!("The value of y after editing: {}", y);

    // constant
    const MAX_VALUE: u32 = 100_000;
    println!("The value of MAX_VALUE is: {}", MAX_VALUE);

    // shadowing
    let z = 5;
    let z = z + 1;
    println!("The value of z is: {}", z);

    // shadowing with different types
    let spaces = "   ";
    let spaces = spaces.len();
    println!("The value of spaces is: {}", spaces);

    // Data Types
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("The value of guess is: {}", guess);

    // ---------------------------------------------------------
    // Scalar Types
    
    // integers 8-bit, 16-bit, 32-bit, 64-bit -> i8, u8, i16, u16, ...
    let scalar_integer: u32 = 5; // unsigned integer
    let scalar_signed_integer: i32 = -5; // signed integer

    // f32, f64
    let scalar_float: f64 = 5.0; // float

    // numeric operation ---
    // addition
    let sum = 5 + 10;
    // subtraction
    let difference = 95.5 - 4.3;
    // multiplication
    let product = 4 * 30;
    // division
    let quotient = 56.7 / 32.2;
    // remainder
    let remainder = 43 % 5;

    println!("The value of sum is: {}, difference {}, multiplication {}, quotient {}, remainder {}", sum, difference, product, quotient, remainder);
    // -----------------------

    let scalar_bool: bool = true; // boolean

    let scalar_char: char = 'A'; // character specified with single quotes, string specified with double quotes
    let c = 'z';
    let z = "z";
    let healt = '❤';
    println!("The value of scalar_char is: {}, c is: {}, z is: {}, healt is: {}", scalar_char, c, z, healt);

    // Compound Types - Can group multiple values into one type
    
    // Tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("The value of tup is: {:?}", tup);
    // destructuring tuple
    let (x, y, z) = tup;
    println!("The value of x is: {}", x);
    // access tuple element by index
    let five_hundred = tup.0;
    println!("The value of x is: {}", five_hundred);

    // Array
    let arr = [1, 2, 3, 4, 5];
    let first = arr[0];
    let second = arr[1];
    println!("The value of first is: {}, second is: {}", first, second);

    


}