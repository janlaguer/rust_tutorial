pub fn run() {
    //Prints to console
    println!("Hello from print.rs");

    let name = "Brad";
    let location = "Mass";
    let activity = "code";

    // formatting can be done this way or by position without the numbers
    println!("That's {1} from {0}", location, name);
    println!("{name} likes to {activity}");

    // Placeholder traits
    println!("Binary {:b}\nHex: {:x}\nOctal: {:o}", 10, 10, 10);

    // Placeholder for debug
    println!("{:?}", (12, true, "hello"));

    // Math
    println!("{}", 10 + 10);
}