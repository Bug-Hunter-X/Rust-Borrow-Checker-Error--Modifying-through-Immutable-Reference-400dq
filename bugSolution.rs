fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    //let z = &x; // z is an immutable reference to x. Commented out to fix the error

    // This is allowed because the compiler can check at compile time
    // that there are no other mutable references to x. 
    *y += 1; 

    println!("x = {}", x);

    //This is now fixed.
    // *z += 1;
} 