// `fmt::Debug` is a trait that implements rudimentary printing capabilities

// This structure cannot be printed either with `fmt::Display` or with 
// `fmt::Debug`
struct UnPrintable(i32);

// The `derive` attribute automatically creates the implementation required
// required to make this `struct` printable with `fmt::Debug`
#[derive(Debug)]
struct DebugPrintable(i32);

// Derive the `fmt::Debug` implementation for `Structure`.
// `Structure` is a structure which contains a single `i32`.
#[derive(Debug)]
struct Structure(i32);

// Put a `Structure` inside of the structure `Deep`.
// Make it printable also.
#[derive(Debug)]
struct Deep(Structure);

// https://doc.rust-lang.org/1.9.0/book/structs.html
#[derive(Debug)]
struct Person<'a>
{
    name: &'a str, // https://doc.rust-lang.org/1.9.0/book/lifetimes.html#in-structs
    age: u8
}
// Any reference to a Person cannot outlive the reference to the `name` field
// that it contains. (How would that happen? I'm not sure.)

fn main()
{
    // We can pass in an instance of `Structure` for printing!
    //
    // Printing with `{:?}` is similar to printing with `{}`.
    println!("1. Now {:?} will print!", Structure(3));
    // Results in...
    /*
        1. Now Structure(3) will print!
    */

    // The problem with `derive` is the lack of control over how the results
    // look. What if I want this to just show a `7`?
    println!("2. Now {:?} will print!", Deep(Structure(7)));
    // Results in...
    /*
        2. Now Deep(Structure(7)) will print!
    */

    println!("3. {:#?}", Person{ name:"Terry", age:21});
    // Results in...
    /*
        3. Person {
        name: "Terry",
        age: 21,
        }
    */
    // Why is this different? See the declaration above for Person!
}