// src/basics.rs

// Struts definition
struct User {
    username: String,
    _email: String,
    _active: bool,
    _trys: u64,
}

enum IpAddrKind {
    V4,
    _V6,
}

pub fn run_basics() {
    println!("=== Rust learning - Basics ===\n");

    // 1. Inmutable variable by default
    let x = 5;
    println!("The value of x is: {}\n", x);
    // x = 6; // This line would cause a compile-time error

    // 2. Mutable variable
    let mut y = 10;
    println!("The value of y is: {}", y);
    y = 20;
    println!("The new value of y is: {}\n", y);

    // 3. Constant
    // They have to be annotated with a especific data type and in uppercase
    // They are immutable by default
    const MAX_POINTS: u32 = 100_000;
    println!("Max points: {}\n", MAX_POINTS);

    // 4. Shadowing
    // You can declare a new variable with the same name as a previous variable
    let spaces = "   ";
    let spaces = spaces.len(); // Now spaces is an integer
    println!("Number of spaces: {}\n", spaces);

    // 5. Data types
    // Tuples
    let tup: (i32, f64, char) = (-21, 2.24, 'B');

    // Destructuring a tuple
    let (x, y, z) = tup;
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}\n", z);

    // Accessing tuple elements by index
    let first_element = tup.0;
    println!("The first element of the tuple is: {}\n", first_element);

    // 6. Arrays
    // Type [i32, 5] mean an array of 5 elements of type i32
    let array = [1, 2, 3, 4, 5];
    let first = array[0];
    println!("The first element of the array id: {}\n", first);

    // 7. Functions
    functions();

    // 8. Control flow
    control_flow();

    // 9. Ownership
    ownership();

    // 10. References and Borrowing
    references_and_borrowing();

    // 11. Structs
    structs();

    // 12. Enums
    enums();

    // 13. Option enum
    option_enums();
}

/* FUNCTIONS */
fn functions() {
    let number = 5;
    let result = sum_five(number);
    println!("The result of adding five to {} is: {}\n", number, result);
}

// Function that takes an integer parameter and returns an integer
fn sum_five(x: i32) -> i32 {
    let intern = x + 1; // Sentece ending with a semicolon does not return a value
    intern + 4 // Last sentece without semicolon returns a value
}
/* FUNCTIONS */

/* CONTROL FLOW */
fn control_flow() {
    let number_conditional: u32 = 6;

    // IF-ELSE
    if number_conditional % 4 == 0 {
        println!("The number is divisible by 4\n");
    } else if number_conditional % 3 == 0 {
        println!("The number is divisible by 3\n");
    } else {
        println!("The number is not divisible by 4 or 3\n");
    }

    // The block if is an expression that could return a value
    let condition = true;
    let a: u32 = if condition { 5 } else { 6 }; // a will be 5 if condition is true, otherwise 6
    println!("The value of a is: {}\n", a);

    // LOOP (infine lopp until break)
    let mut count: u32 = 0;
    loop {
        count += 1;
        if count == 3 {
            println!("Reached count of 3, breaking the loop.\n");
            break;
        }
    }

    // WHILE (race against a condition)
    let mut n = 3;
    while n > 0 {
        println!("n is: {}", n);
        n -= 1;
    }
    println!("");

    // FOR (iterate over a collection) must use iterators
    let a_array: [u32; 3] = [10, 20, 30];
    for element in a_array {
        println!("The value is: {}", element);
    }
    println!("");

    // FOR with RANGE (last element not included)
    for number in 1..4 {
        println!("Range: {}", number);
    }
    println!("");
}
/* CONTROL FLOW */

/* OWNERSHIP */
fn ownership() {
    let s1 = String::from("Hello"); // s1 is the owner of "Hello" string (memory allocated in heap)
    let s2 = s1; // MOVEMENT of ownership from s1 to s2, s1 is no longer valid
    // println!("{}", s1); // This line would cause a compile-time error
    println!("s2 is: {}\n", s2);
}
/* OWNERSHIP */

/* REFERENCES AND BORROWING */
fn references_and_borrowing() {
    let s1 = String::from("Hello, Rust!");

    // We pass a reference (&s1), not the ownership of s1
    let len = calculate_length(&s1); // Pass a reference to s1
    println!("The length of '{}' is {}.\n", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len() // Return the length of the string
} // Here s goes out of scope, but since we only borrowed it, nothing happens
/* REFERENCES AND BORROWING */

/* STRUCTS */
fn structs() {
    // Create an instance of the User struct
    let user1: User = User {
        _email: String::from("ty@gmail.com"),
        username: String::from("Ty"),
        _active: true,
        _trys: 1,
    };

    println!("User: {}\n", user1.username);
}
/* STRUCTS */

/* ENUMS */
fn enums() {
    let four = IpAddrKind::V4;
    route(four);
}

fn route(ip_kind: IpAddrKind) {
    match ip_kind {
        IpAddrKind::V4 => println!("Routing an IPv4 address.\n"),
        IpAddrKind::_V6 => println!("Routing an IPv6 address.\n"),
    }
}
/* ENUMS */

/* OPTION ENUMS */
fn option_enums() {
    let some_number = Some(5);
    let _some_string = Some("a string");
    let _absent_number: Option<i32> = None;

    // Get a value out of an Option enum using match safely
    match some_number {
        Some(i) => println!("We have number: {}\n", i),
        None => println!("No number found.\n"),
    }
}
/* OPTION ENUMS */
