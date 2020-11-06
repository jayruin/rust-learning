fn main() {
    // String type can be mutated
    // It goes on the heap instead of the stack
    // let mut s = String::from("Hello");
    // s.push_str(", world!");
    // println!("{}", s);

    // When variables go out of scope, drop is called,
    // this deallocates resources
    // let s1 = String::from("Hello");
    // let s2 = s1;
    // After previous line, s1 goes out of scope
    // This way, s1 and s2 won't both go out of scope
    // and try to free the same memory
    // println!("{}, world!", s1); // Should get error

    // There won't be an error here
    // let s2 = s1.clone();
    // println!("s1 = {}, s2 = {}", s1, s2);

    // Integers are stored on the stack so no errors
    // let x = 5;
    // let y = x;
    // println!("x = {}, y = {}", x, y);
    // Integers can have a Copy trait which would
    // allow the previous variable to be still usable
    // after assignment. However, types that have
    // implemented Drop trait cannot have Copy trait

    // Ownership and Functions
    let s = String::from("Hello");
    take_ownership(s); // s is no longer valid here
    let x = 5;
    makes_copy(x); // i32 has Copy trait so it is still valid

    // Returning Values
    // let s1 = gives_ownership(); // return value moved into s1
    // let s2 = String::from("Hello"); // s2 comes into scope
    // let s3 = takes_and_gives_back(s2); // s2 moves into takes_and_gives_back, then moves into s3

    // Multiple return values
    // We want to have a function use a value, but not take ownership
    let s1 = String::from("Hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}", s2, len);
    // Note that we cannot call s1 since it was moved into s2!
    // This is tedious, we will instead use references later
}
// s1, s3 go out of scope and are dropped
// s2 goes out of scope, but since it was moved, nothing happens

fn take_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
}
// some_string goes out of scope and "drop" is called.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
}
// some_integer goes out of scope

fn gives_ownership() -> String {
    let some_string = String::from("Hello");
    some_string // some_string is returned and moves out to the calling function
}

fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into scope
    a_string // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}