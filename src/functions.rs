pub fn run() {
    greeting("Hello", "my guy");

    // Bind function values to variables
    let get_sum = add(5, 3);
    println!("Sum: {get_sum}");

    //Closures
    // it's possible to get variables from outside the function
    let z = 2;
    let add_nums = |x: i32, y: i32| x + y + z;
    println!("C Sum: {}", add_nums(3, 9))

}

fn greeting(greet: &str, name: &str) {
    println!("{greet} {name}, nice to meet you!")
}

fn add(x: i32, y: i32) -> i32 {
    x + y // no semi colons means you're returning this value
}