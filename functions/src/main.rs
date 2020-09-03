fn main() {
    // another_function(5, 6);
    // let x = (let y = 6); // Error since statements do not return anything
    // Block for new scopes is an expression
    // let x = 5;
    let y = {
        let x = 3;
        x + 1 // Note the lack of semicolon for expressions
    };
    println!("The value of y is {}", y);
}

// fn another_function( x: i32, y: i32) {
//     println!("The value of x is {}", x);
//     println!("The value of y is {}", y);
// }