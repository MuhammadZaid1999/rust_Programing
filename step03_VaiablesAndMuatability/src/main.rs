fn main() {
    // ---------------------- MUATABILITY ---------------------
    let x = 5;
    println!("The value of x is: {x}");
    x = 6;  // Error: cannot assign twice to immutable variable
    println!("The value of x is: {x}");

    let mut y = 5;
    println!("The value of y is: {y}");
    y = 6;
    println!("The value of y is: {y}");


    // ---------------------- CONSTANTS ---------------------
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("THREE_HOURS_IN_SECONDS is: {THREE_HOURS_IN_SECONDS}");


    // ---------------------- SHADOWING ---------------------
    let z = 5;
    let z = z - 2;
    {
        let z = z * 2;
        println!("The value of z in the inner scope is: {z}");
    }
    println!("The value of z is: {z}");


    let spaces = "   ";
    let spaces = spaces.len();
    println!("The value of spaces is: {spaces}");

    let mut spaces1 = "   ";
    spaces1 = spaces1.len(); // error: could not compile due to 1 previous error
    

}
