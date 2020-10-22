fn main() {
    // String type can be mutated
    // It goes on the heap instead of the stack
    let mut s = String::from("Hello");
    s.push_str(", world!");
    println!("{}", s);

    // When variables go out of scope, drop is called,
    // this deallocates resources
    let s1 = String::from("Hello");
    // let s2 = s1;
    // After previous line, s1 goes out of scope
    // This way, s1 and s2 won't both go out of scope
    // and try to free the same memory
    // println!("{}, world!", s1); // Should get error

    // There won't be an error here
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    // Integers are stored on the stack so no errors
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);
    // Integers can have a Copy trait which would
    // allow the previous variable to be still usable
    // after assignment. However, types that have
    // implemented Drop trait cannot have Copy trait
}
