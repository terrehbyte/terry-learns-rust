//
// MAIN
//

// scalar types
// - signed integers
//   - i8, i16, i32, i64, i128, isize (pointer size)
// - unsigned integers
//   - u8, u16, u32, u64, u128, usize (pointer size)
// - floating point
//   - f32, f64
// - char, unicode letters
//   - 'a', 'α', '∞'
// - bool
//   - true/false
// - unit type (), whose only possible value is an empty tuple

fn main()
{

    //
    // VARIABLES
    //

    // VARIABLES CAN BE TYPE ANNOTATED
    // let $identifier : $type = $value;
    let logical: bool = true;

    let a_float: f64 = 1.0; // Regular annotation
    let an_integer = 5i32; // Suffix annotation

    // VARIABLE TYPE DEFAULTS
    let default_float = 3.0; // `f64`
    let default_integer = 7; // `i32`

    // TYPE INFERRED FROM CONTEXT
    let mut inferred_type = 12; // Type i64 is inferred from another context
    inferred_type = 4294967296i64;

    // ...

    // A mutable variable's value can be changed
    let mut mutable = 12; // mutable `i32`
    mutable = 21;

    // Error! The type of a variable can't be changed!
    // mutable = true;

    // Variables can be overwritten with shadowing.
    let mutable = true;

    //
    // LITERALS
    //

    let litInt = 5i8;
    let litUnsigned = 21u8;
    let litFlt = 0.1f32;
    let litChar = 't';
    let litBool = true;

    let arrInt = [1, 2, 3]; // literal integer array
    let litTup = (1, true); // literal tuple (i32, bool)

    for num in arrInt.iter()
    {
        println!("{}", *num);
    }

    //
    // OPERATOR
    //

    // Integer addition
    println!("1 + 2 = {}", 1u32 + 2);

    // Integer subtract
    println!("Signed:   1 - 2 = {}", 1i32 - 2);
    // println!("Unsigned: 1 - 2 = {}", 1u32 - 2); // compile error: can't underflow

    // Short-circuiting boolean logic
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);
}