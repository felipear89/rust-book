fn shadowing() {
    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("[shadowing] The value of x is: {}", x);

    // THIS IS NOT POSSIBLE
    // let mut spaces = "   ";
    // spaces = spaces.len();


    let spaces = "   ";
    let spaces = spaces.len();
    println!("{}", spaces)
}

fn data_types() {
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("u32: {}", guess);

    // floating points
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    println!("{} + {} = {}",x,y, x+y);

    // Numeric operations
    // addition
    let _sum = 5 + 10;
    // subtraction
    let _difference = 95.5 - 4.3;
    // multiplication
    let _product = 4 * 30;
    // division
    let _quotient = 56.7 / 32.2;
    // remainder
    let _remainder = 43 % 5;

    // boolean types
    let _t = true;
    let _f: bool = false; // with explicit type annotation
    
    // Rust’s char type is four bytes in size and represents a Unicode Scalar Value
    // character type
    let _c = 'z';
    let _z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("{}", heart_eyed_cat);

    // Tuple
    // A tuple is a general way of grouping together a number of values with a variety of types into one compound type. 
    // Tuples have a fixed length: once declared, they cannot grow or shrink in size.
    // Each position in the tuple has a type, and the types of the different values in the tuple don’t have to be the same.
    let tup = (500, 6.4, 1);
    let (_t1, t2, _t3) = tup;
    println!("The value of t2 is: {}", t2);
    println!("The value of position 0 is: {}", tup.0);

    // Array
    // Unlike a tuple, every element of an array must have the same type
    // arrays in Rust have a fixed length
    // Arrays are useful when you want your data allocated on the stack rather than the heap

    let my_array: [i32; 5] = [1, 2, 3, 4, 5]; // [type; size]
    let _same_value_array = [3; 5]; // 3,3,3,3,3
    println!("my_array[3] = {}", my_array[2]);

}

fn main() {
    const MAX_POINTS: u32 = 100_000;
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    println!("{}", MAX_POINTS);

    shadowing();
    data_types();
    
}
