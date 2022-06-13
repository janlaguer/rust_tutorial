pub fn run() {
    let mut count = 0;

    // similar to while True but no conditional 
    loop {
        count += 1;
        println!("{}", count);

        if count == 20 {
            break;
        }
    }

    count = 1;
    // While loop (FizzBuzz)
    while count <= 15 {
        let mut output = String::with_capacity(10);
        if count % 3 == 0 {
            output.push_str("Fizz")
        }
        if count % 5 == 0 {
            output.push_str("Buzz")
        }
        
        if output.len() > 0 {
            println!("{}", output);
            count += 1;
            continue
        }
        println!("{}", count);
        count += 1;
    }

    // For loop (FizzBuzz) 
    // loops through an iterator (arrays, vectors, etc.)
    for x in 1..16 {
        let mut output = String::with_capacity(10);
        if x % 3 == 0 {
            output.push_str("Fizz")
        }
        if x % 5 == 0 {
            output.push_str("Buzz")
        }
        
        if output.len() > 0 {
            println!("{}", output);
            continue
        }
        println!("{}", x);
    }
}