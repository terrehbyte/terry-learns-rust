
// Import (via `use`) the `fmt` module to make it available.
use std::fmt;

//
// STRUCTURE
//

// Define a structure for which `fmt::Display` will be implemented. This is
// a tuple struct named `Structure` that contains an `i32`.
struct Structure(i32);

// To use the `{}` marker, the trait `fmt::Display` must be implemented
// manually for the type.
impl fmt::Display for Structure
{
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}", self.0)
    }
}

//
// RANGE
//

#[derive(Debug)]
struct Range(i64, i64); // unnamed, so refer to them by position

impl fmt::Display for Range
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        // self.number refers to each positional data point
        write!(f, "({}, {})", self.0, self.1)
    }
}

//
// Point2D
//

#[derive(Debug)]
struct Point2D
{
    x: f64,
    y: f64
}

impl fmt::Display for Point2D
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        // Customize so only `x` and `y` are denoted
        write!(f, "{}, {}", self.x, self.y)
    }
}

//
// CITY
//

use std::fmt::{Formatter, Display};

struct City
{
    name: &'static str,
    // latitude
    lat: f32,
    // longitude
    lon: f32
}

impl Display for City
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result
    {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

        // write! is like format! but it writes the formatted into the buffer.
        // the buffer is specified as the first argument
        write!(f, "{}: {:.3}°{} {:.3}°{}",
               self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
    }
}

//
// COLOR
//

#[derive(Debug)]
struct Color
{
    red: u8,
    green: u8,
    blue: u8
}

impl Display for Color
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result
    {
        // Customize so only `x` and `y` are denoted
        // https://stackoverflow.com/a/50458236/2787129
        write!(f, "RGB ({R}, {G}, {B}) 0x{R:0>2X}{G:0>2X}{B:0>2X}",
               R=self.red, G=self.green, B=self.blue)
    }
}

//
// MAIN
//

fn main()
{
    println!("Let's print a Structure, but pretty!");
    println!("{}", Structure(5));

    let range = Range(0, 14);

    println!("Compare the following two structures:");
    println!("Display: {}", range); // (0, 14)
    println!("Debug: {:?}", range); // Range(0, 14)

    let big_range = Range(-300,300);
    let lil_range = Range(-3, 3);

    println!("The big range is {big} and the lil is {lil}",
             big = big_range,
             lil = lil_range);

    let point = Point2D { x: 3.3, y: 7.2 };

    println!("Compare points:");
    
    // Displaying a Point2D should use our user-defined impl above
    println!("Display: {}", point); // 3.3, 7.2
    println!("Debug: {:?}", point); // Point2D { x: 3.3, y: 7.2 }

    // Printing w/ {:b} specifier will require that `fmt::Binary` be implemented
    
    // ...

    // Print an array of cities

    // for each element in this collection (referred to as city)...
    for city in [
        City { name: "Dublin",    lat: 53.347778, lon: -6.259722 },
        City { name: "Oslo"  ,    lat: 59.95    , lon: 10.75 },
        City { name: "Vancouver", lat: 49.25,     lon: -123.21 },
    ].iter() // iterate over them and print them
    {
        // deference
        println!("{}", *city);
    }

    for col in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0,   green: 3,   blue: 254 },
        Color { red: 0,   green: 0,   blue: 0 }
    ].iter()
    {
        println!("{}", *col);
    }
}