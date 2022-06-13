/*
    Primitives:
    u32: unsigned types just mean positive numbers
    i32: can be negative number
    f32: decimal numbers
    bool: true or false
    char: just one character like j
    tuples: kinda like lists
    array: fixed length list
*/

pub fn run() {
    // types can be infered
    //default to 32 bit of the type
    let i = 20; //i32
    let f = 2.5; //f32
    let u: u64 = 10; // explicitly say its unsigned with 64 bits

    println!("Max of i32 is {}", std::i32::MAX); //highest number you can get from 32bits
    println!("Max of i64 is {}", std::i64::MAX); //higher number you can get from 64

    // setting char
    let bruh = 'b'; // single quotes mean char; as long as its one unicode
    let face: char = '\u{1F91D}'; // handshake emoji

    println!("{:?}", (i, f, u, bruh, face))
}