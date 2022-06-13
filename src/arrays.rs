// Arrays are fixed in length and data type

pub fn run() {
    let mut numbers: [i32; 5] = [1,2,3,4,5];

    // Reassign numbers
    numbers[1] = 20;

    println!("{:?}", numbers);
    println!("Single Value: {}", numbers[4]);
    println!("New slice: {:?}", &numbers[0..2]);
}