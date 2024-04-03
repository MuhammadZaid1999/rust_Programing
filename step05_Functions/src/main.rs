fn main() {
    println!("Hello, world!");

    another_function();
    another_function1(5);
    print_labeled_measurement(5, 'h');

    // Statements and Expressions
    // Statements are instructions that perform some action and do not return a value.
    // Expressions evaluate to a resultant value. Let’s look at some examples.
    
    let y = 6;  // A main function declaration containing one statement
    
    // let x = (let y = 6);  // Statements do not return values. Therefore, you can’t assign a let statement to another variable, as the following code tries to do; you’ll get an error:

    // This expression:
    // is a block that, in this case, evaluates to 4. That value gets bound to x as part of the let statement. Note that the z + 1 line doesn’t have a semicolon at the end, which is unlike most of the lines you’ve seen so far. Expressions do not include ending semicolons. If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value. Keep this in mind as you explore function return values and expressions next.
    let x = {
        let z = 3;
        z + 1
    };
    println!("The value of x is: {x}");

    let z = five();
    println!("The value of z is: {z}");

    let a = plus_one(5);
    println!("The value of a is: {a}");
    
}

fn another_function(){
    println!("Another function.");
}

fn another_function1(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
    // x + 1; But if we place a semicolon at the end of the line containing x + 1, changing it from an expression to a statement, we’ll get an error:
    // The main error message, mismatched types, reveals the core issue with this code. The definition of the function plus_one says that it will return an i32, but statements don’t evaluate to a value, which is expressed by (), the unit type. Therefore, nothing is returned, which contradicts the function definition and results in an error. In this output, Rust provides a message to possibly help rectify this issue: it suggests removing the semicolon, which would fix the error.
}
