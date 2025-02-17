fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    let z = &x; // z is an immutable reference to x

    // This is allowed because the compiler can check at compile time
    // that there are no other mutable references to x. 
    *y += 1; 

    println!("x = {}", x);

    //This will result in compile time error. 
    *z += 1;
}