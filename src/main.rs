fn main() {
    println!("Hello, world!");
}

// The arrow (token) precedes the return type. Our function returns a u64 value.
fn _gcd(mut n: u64, mut m: u64) -> u64 {
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

// Writing and Running Unit Tests

// The below definition mark test_gcd as a test function, to be skipped in normal compilations, but included and called automatically if we run our program with cargo test.
#[test]
// Defining a function which calls gcd and checks that it returns correct values.
fn test_gcd() {
    assert_eq!(_gcd(14, 15), 1);

    assert_eq!(_gcd(2*3*5*11*17, 3*7*11*13*19), 3*11);
}