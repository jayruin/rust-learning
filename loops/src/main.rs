fn main() {
    // Infinite loop
    // loop {
    //     println!("Again!");
    // }

    // Returning values from loops
    // let mut counter = 0;
    // let result = loop {
    //     counter += 1;
    //     if counter == 10 {
    //         break counter * 2;
    //     }
    // };
    // println!("The result is {}", result);

    // Conditional loop with while
    // let mut number = 3;
    // while number != 0 {
    //     println!("{}!", number);
    //     number -= 1;
    // }
    // println!("LIFTOFF!!!");
    // Looping with for
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");

    // Looping through a collection with for
    // let a = [10, 20, 30, 40, 50];
    // While loop
    // let mut index = 0;
    // while index < 5 {
    //     println!("the value is: {}", a[index]);
    //     index += 1;
    // }
    // For loop
    // for element in a.iter() {
    //     println!("the value is: {}", element);
    // }
}
