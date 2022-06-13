/*
Strings can be either:
primitive str: immutable
String: growable, mutable
*/

pub fn run() {
    // let hello = "hello"; //primitive str
    let mut hello2 = String::from("Hello"); //growable string

    println!("{}", hello2);

    // getting the length
    println!("{} Length: {}", hello2, hello2.len());

    hello2.push('W'); // can only push chars

    println!("{} Length: {}", hello2, hello2.len());

    hello2.push_str("orld"); // this pushes str

    println!("{} Length: {}", hello2, hello2.len());

    println!("{} capacity: {}", hello2, hello2.capacity());
    println!("{} is empty? {}", hello2, hello2.is_empty());
    println!("{} contains \"World\": {}", hello2, hello2.contains("World"));

    hello2 = hello2.replace("World", " World");
    println!("New Word: {}", hello2);
    // Looping through string by whitespace
    for word in hello2.split_whitespace() {
        println!("{}", word);
    }

    // Assertion testing
    assert_eq!(11, hello2.len()) // does nothing if pass but prints to console if doesn't pass
}