fn main() {
    // let number = 3;
    // let number = 7;
    // let number = 6;

    // if number < 5 {
    //     println!("Condition was true")
    // } else {
    //     println!("Condition was false");
    // }

    // No implicit conversion to bool
    // if number {
    //     println!("Number was three");
    // }
    // if number != 0 {
    //     println!("Number was something other than zero");
    // }

    // Multiple conditions
    // if number %4 == 0 {
    //     println!("Number is divisible by 4");
    // } else if number % 3 == 0 {
    //     println!("Number is divisible by 3");
    // } else if number % 2 == 0 {
    //     println!("Number is divisible by 2");
    // } else {
    //     println!("Number is not divisible by 4, 3, or 2");
    // }

    // Using if in a let statement
    let condition = true;
    // let number = if condition { 5 } else { 6 };
    // Mismatched types, won't compile
    let number = if condition { 5 } else { "six" };
    println!("The value of number is {}", number);
}
