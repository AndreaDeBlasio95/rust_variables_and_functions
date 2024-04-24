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
    println!("The value of scalar_integer is: {}, scalar_signed_integer is: {}", scalar_integer, scalar_signed_integer);

    // f32, f64
    let scalar_float: f64 = 5.0; // float
    println!("The value of scalar_float is: {}", scalar_float);

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
    println!("boolean value is: {}", scalar_bool);

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
    println!("The value of x is: {}, y is: {}, z is: {}", x, y, z);
    // access tuple element by index
    let five_hundred = tup.0;
    println!("The value of x is: {}", five_hundred);

    // Array
    let arr = [1, 2, 3, 4, 5];
    let first = arr[0];
    let second = arr[1];
    println!("The value of first is: {}, second is: {}", first, second);

    // Functions
    another_function();
    
    function_with_parameter(5);
    
    function_with_two_parameters(5, 6); 
    
    let my_x = function_with_return_type();
    println!("The value of my_x is: {}", my_x);

    let x_plus_one = plus_one(5);
    println!("The value of x_plus_one is: {}", x_plus_one);

    // Control Flow
    control_flow_topic();

    // Loops
    loops_topic();

}

fn another_function() {
    println!("Another function.");
}

fn function_with_parameter(x: i32) {
    println!("The value of x is: {}", x);
}

fn function_with_two_parameters(x: i32, y: i32) {
    println!("The value of x is: {}, y is: {}", x, y);
}

fn function_with_return_type() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn control_flow_topic () {

    let number = 6;

    if number % 4 == 0 {
        println!("divisible by 4");
    } else if number % 3 == 0 {
        println!("divisible by 3");
    } else {
        println!("not divisible by 4 or 3");
    }

    // using if in a let statement
    let condition = true;
    let my_number = if condition {
        5
    } else {
        6
    };
    println!("The value of my_number is: {}", my_number);
}

fn loops_topic () {
    loop {
        println!("Infinite loop - Stop killing the terminal or with CTRL-C");
    }
}