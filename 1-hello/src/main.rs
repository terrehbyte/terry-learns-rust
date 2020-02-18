// This is a comment!

/*
    This is a block comment!
*/

#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

/// Three forward slashes can be used to generate documentation for the
/// following item.
fn main() {

    //! Alternatively, you can use two forward slashes and an exclaimation mark
    //! to generate documentation for the enclosing item.
    //! 
    //! In VS Code, the three forward slashes seems to take precedence over the
    //! enclosing approach.

    // prints a message to stdout
    print!("Hello, line");

    // same as `print!` but also appends a newline
    println!("Hello, new line");

    // writes formatted text to a string
    // go here for format traits: https://doc.rust-lang.org/std/fmt/#formatting-traits
    let name = format!("{} {}", "Terry", "Nguyen");
    let fullName = format!("My name is {0} {1}, or {0} for short.", "Terry", "Nguyen");
    
    println!("You can have named arguments, like fruit: {fruit}", fruit="orange");

    println!("You can also use format trits, like b for binary: {:b}", 2);
}

/*
    This file can be compiled by providing to the compiler via CLI, like so:

    rustc main.rs

*/