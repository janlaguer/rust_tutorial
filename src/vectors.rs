// vectors are like lists in python; similar to arrays

pub fn run() {
    let mut numbers: Vec<i32> = vec![1,2,3,4,5];

    numbers[1] = 20;

    // pushing new numbers
    numbers.push(6);

    // popping last value
    numbers.pop();

    println!("{:?}", numbers);
    println!("Single Value: {}", numbers[4]);
    println!("New slice: {:?}", &numbers[0..2]);

    for i in numbers.iter() {
        println!("Number: {}", i)
    }

    for i in numbers.iter_mut() {
        *i *= 10;
    }

    println!("New: {:?}", numbers)
}