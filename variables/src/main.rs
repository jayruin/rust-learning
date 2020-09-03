fn main() {
    // error: cannot assign twice to immutable variable
    // let x = 5;
    // No error
    // let mut x = 5;
    // println!("The value of x is: {}", x);
    // x = 6;
    // println!("The value of x is: {}", x);

    // const MAX_POINTS: u32 = 100_000;

    // Shadowing
    // let x = 5;
    // let x = x + 1;
    // let x = x * 2;
    // println!("The value of x is: {}", x);

    // Shadowing vs mut
    // let spaces = "    ";
    // let spaces = spaces.len();
    // let mut spaces = "    ";
    // spaces = spaces.len();

    // Need to add type annotation, otherwise error
    // let guess: u32 = "42".parse().expect("Not a number!");

    // Floating-point types
    // let x = 2.0;
    // let y: f32 = 3.0;

    // Numeric operations
    // let sum = 5 + 10;
    // let difference = 95.5 - 4.3;
    // let quotient = 56.7 / 32.2;
    // let remainder = 43 % 5;

    // Boolean type
    // let t = true;
    // let f: bool = false;

    // Character type
    // let c = 'z';
    // let z = 'Z';
    // let heart_eyed_cat = '😻';

    // Tuple
    // let tup: (i32, f64, u8) = (500, 6.4, 1);
    // let (x, y, z) = tup; // Destructuring
    // println!("The value of y is: {}", y);
    // let x: (i32, f64, u8) = (500, 6.4, 1);
    // let five_hundred = x.0;
    // let six_point_four = x.1;
    // let one = x.2;

    // Array
    // let a: [i32; 5] = [1, 2, 3, 4, 5];
    // let first = a[0];
    // let second = a[1];
    // let index = 10;
    // let element = a[index];
    // println!("The value of element is {}", element);
    // let a = [3; 5];
    // let a = [3, 3, 3, 3, 3];
}
