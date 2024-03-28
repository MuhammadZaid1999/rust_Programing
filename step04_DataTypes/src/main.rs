fn main() {
    // Every value in Rust is of a certain data type, which tells Rust what kind of data is being specified so it knows how to work with that data. We’ll look at two data type subsets: scalar and compound.

    let guess: u32 = "42".parse().expect("Not a number!");
    print!("{guess}");

    // let guess: u32 = "aa".parse().expect("Not a number!");
    // print!("{guess}");
    // this code print "Not a number!"

    // let guess = "42".parse().expect("Not a number!");
    // type annotations needed ----- type must be known at this point

    // let guess: _ = "42".parse().expect("Not a number!");
    // type annotations needed ---- cannot infer type of the type parameter `F` declared on the method `parse`


    // ------------------------ Scalar Types ---------------------------
    // A scalar type represents a single value. Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters

    // Integer Types
    // Length	Signed	Unsigned
    // 8-bit	i8	    u8
    // 16-bit	i16	    u16
    // 32-bit	i32	    u32
    // 64-bit	i64	    u64
    // 128-bit	i128	u128
    // arch 	isize	usize

    // Each signed variant can store numbers from -(2n - 1) to 2n - 1 - 1 inclusive, where n is the number of bits that variant uses. So an i8 can store numbers from -(27) to 27 - 1, which equals -128 to 127. Unsigned variants can store numbers from 0 to 2n - 1, so a u8 can store numbers from 0 to 28 - 1, which equals 0 to 255.

    // Additionally, the isize and usize types depend on the architecture of the computer your program is running on, which is denoted in the table as “arch”: 64 bits if you’re on a 64-bit architecture and 32 bits if you’re on a 32-bit architecture.

    // You can write integer literals in any of the forms shown in Table 3-2. Note that number literals that can be multiple numeric types allow a type suffix, such as 57u8, to designate the type. Number literals can also use _ as a visual separator to make the number easier to read, such as 1_000, which will have the same value as if you had specified 1000.
    // Number literals	    Example
    // Decimal	            98_222
    // Hex	                0xff
    // Octal	            0o77
    // Binary	            0b1111_0000
    // Byte (u8 only)	    b'A'
    let a: u32 = 98_222;
    let b: u16 = 0xff;
    let c: u16 = 0o77;
    let d: u16 = 0b1111_0000;
    let e: u8 = b'A';
    print!("{a} {b} {c} {d} {e}");

    
    // Floating-Point Types
    // Rust also has two primitive types for floating-point numbers, which are numbers with decimal points. Rust’s floating-point types are f32 and f64, which are 32 bits and 64 bits in size, respectively. The default type is f64 because on modern CPUs, it’s roughly the same speed as f32 but is capable of more precision. All floating-point types are signed.
    let x = 2.55501230; // f64
    print!("\n{x}");
    let y: f32 = 3.5551230; // f32
    print!("\n{y}");
    // Floating-point numbers are represented according to the IEEE-754 standard. The f32 type is a single-precision float, and f64 has double precision.

    
    // Numeric Operations
    // Rust supports the basic mathematical operations you’d expect for all the number types: addition, subtraction, multiplication, division, and remainder. Integer division truncates toward zero to the nearest integer
    // addition
    let sum = 5 + 10;
    print!("\n{sum}");

    // subtraction
    let difference = 95.5 - 4.3;
    print!("\n{difference}");

    // multiplication
    let product = 4 * 30;
    print!("\n{product}");

    // division
    let quotient = 56.7 / 32.2;
    print!("\n{quotient}");

    let truncated = -5 / 3; // Results in -1
    print!("\n{truncated}");

    // remainder
    let remainder = 43 % 5;
    print!("\n{remainder}");


    // The Boolean Type
    // As in most other programming languages, a Boolean type in Rust has two possible values: true and false. Booleans are one byte in size. The Boolean type in Rust is specified using bool. For example:
    let t = true;
    let f: bool = false; // with explicit type annotation


    // The Character Type
    // Rust’s char type is the language’s most primitive alphabetic type. Here are some examples of declaring char values:
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';



    //---------------------------- Compound Types -------------------------
    // Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.

    // The Tuple Type
    // A tuple is a general way of grouping together a number of values with a variety of types into one compound type. Tuples have a fixed length: once declared, they cannot grow or shrink in size.
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (a1, b1, c1) = tup;
    print!("\n {b1}");

    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("\nThe value of y is: {y}");

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("\n {one}");


    // The Array Type
    // Another way to have a collection of multiple values is with an array. Unlike a tuple, every element of an array must have the same type. Unlike arrays in some other languages, arrays in Rust have a fixed length.
    let a = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June", "July",
    "August", "September", "October", "November", "December"];

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    // Here, i32 is the type of each element. After the semicolon, the number 5 indicates the array contains five elements.

    let a = [3; 5];
    // The array named a will contain 5 elements that will all be set to the value 3 initially. This is the same as writing let a = [3, 3, 3, 3, 3]; but in a more concise way.

    // accessing array
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
              

}
