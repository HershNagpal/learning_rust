fn main() {
    let x = 5;
    println!("The value of x is {}.", x);
    // Value of x cannot change because variables are immutable
    let mut y = 3;
    println!("The value of y is {}", y);
    y = 4;
    println!("The value of y is now {} becuase it is mutable.", y);
}
