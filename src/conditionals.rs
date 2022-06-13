pub fn run() {
    let age: u8 = 18;
    let check_id: bool = false;

    // use || for the "or" operator

    if age >= 21 && check_id {
        println!("They can drink")
    }
    else if age < 21 && check_id{
        println!("Can't drink")
    }
    else {
        println!("I'll need to see some ID")
    }

    // short-hand 

    let is_of_age = if age >= 21 {true} else {false};

    println!("Is of age? {}", is_of_age)
}