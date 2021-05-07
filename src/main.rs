// Handling Command-Line Arguments (Part 3)

// The 'use' declaration brings the two traits Write and FromStr into scope. A trait is a collection of methods that types can implement. Although we never use Write of FromStr in the program, a trait must be in scope in order to use its methods.
// Any type that implements the Write trait has a write_fmt method that writes formatted text to a stream. the std::io::strderr type implements Write, and we'll use the writeIn! macro to print error msgs; that macro expands to code that uses the write_fmt method.
use std::io::Write;
// Any type that implements the FromStr trait has a from_str method that tries to parse a value of that type from a string. The u64 type implements FromStr, and we'll call u64::from_str to parse our command-line arguments.
use std::str::FromStr;

// Main function doesn't return a value so we can omit the ->
fn main() {
    // We create a mutable variable set to a new Vec or vector which is the same as Python's list or JS's array. We must make the variable mut even though Vec is designed to be modified do to Rust's inherent vars are immutable unless stated otherwise.
    // We don't need to specify u64 here because Rust will infer it via gcd because that function only accepts u64 values.
    let mut numbers = Vec::new();

    // Here is a standard for loop in which the var arg takes on the arguments being looped over
    // std::env::args returns an iterator, a value that produces each argument on demand, and indicates when we're done. Iterators are things that'd we normally want to loop over and Rust's standard library contains many types.
    // The first value produced by std::... is the name of the program being run, so we skip() over it.
    for arg in std::env::args().skip(1) {
        // Here we call u64::from_str to attempt to parse our command-line argument arg as an unsigned 64-bit integer. The from_str function doesn't return a u64 directly, but rather a Result value that indicates whether the parse succeeded or failed. A Result Value is one of two variants:
        // Ok(v), indicating that the parse succeeded and v is the value produced.
        // Err(e), indicating that the parse failed and e is an error value explaining why.
        // Rust does not have exceptions: all errors are handled using either Result or panic.
        // We check the success of our parse by using the expect method. If an error, expect prints a msg that includes a description of e, and exists the program immediately. However if Ok(v), expect returns v itself, which we push onto the end of our vector of numbers.
        numbers.push(u64::from_str(&arg).expect("error parsing argument"));
    }

    // We check that the length of our numbers vector isn't 0 as we don't want to divide by zero, if it does, we want to exit the program.
    if numbers.len() == 0 {
        // writeLn! macro allows us to write  our error message to the standard error output stream provided by std::io:stderr().
        // The .unwrap() call is a terse way to check that the attempt to print the error msg did not itself fail.
        writeln!(std::io::stderr(), "Usage: gcd NUMBER ...").unwrap();
        std::process::exit(1);
    }

    // Making var d mutable as its value will change. Initially setting it to the first value of the vector.
    let mut d = numbers[0];
    // The & operator borrows a reference to the vector's elements from the second onward. The for loop iterates over the referenced elements, letting m borrow each element in succession.
    for m in &numbers[1..] {
        // The * operator dereferences m, yielding the value it refers to; this is the next u64 we want to pass to gcd. This will be explained in detail in later chapters. But essentially:
        // &x borrows a reference to x, and that *r is the value that the reference r refers to.
        // Since numbers owns the vector, Rust automatically frees it when numbers goes out of scope at the end of main.
        d = gcd(d, *m);
    }

    println!("The greatest common divisor of {:?} is {}", numbers, d);

    // Rust assumes that if main returns at all, the program finished successfully. Only by explicitly calling functions like expect or std::process::exit can we cause the program to terminate with an error status code.

    // We can run the program from cmd line cargo run 42 56 or 42 56 80, or 42, or none at all.
}

// A Simple Function (part 1)

// The arrow (token) precedes the return type. Our function returns a u64 value.
fn gcd(mut n: u64, mut m: u64) -> u64 {
    // assert! is a marco that verifies that neither argument is zero. The ! character marks this as a macro invocation, not a function call. assert! checks that its argument is true, and if it is not, terminates the program (called a panic).
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            // Rust only infers type within function bodies. We must write out the types of function parameters (as above) and return values (as above).
            // If we wanted to specify t, let t: u64 = m;
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    // Rust has a return statement, but we don't need one here. If a function body ends with an expression that is NOT followed by a semicolon, that's the function's return value. In fact, any block surrounded by curly braces can function as an expression. Ex:
    // {
    //     println!("evaluating cos x");
    //     x.cos()
    // }
    // The above is an expression that prints a message then yields x.cos() as it's value.
    // It's typical in Rust to use this form to establish the function's value when control "falls off the end" of the function, and use return statements only for explicit early returns from the midst of a function.
    n
}

// Writing and Running Unit Tests (part 2)

// The below definition mark test_gcd as a test function, to be skipped in normal compilations, but included and called automatically if we run our program with cargo test.
// #[test] is called an attribute. Attributes are an open-ended system for marking functions and other declarations with extra info. They're used to control compiler warnings and code style checks, include code conditionally, tell Rust how to interact with code written in other languages, etc.
#[test]
// Defining a function which calls gcd and checks that it returns correct values.
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);

    assert_eq!(gcd(2*3*5*11*17, 3*7*11*13*19), 3*11);
}
