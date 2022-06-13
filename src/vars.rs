pub fn run() {
    let name = "Brad";
    let mut age = 21;
    println!("My name is {name} and I'm {age}");

    age = 22;
    println!("My name is {name} and I'm now {age}");

    const ID: i32 = 1_000; // "_" is similar to , when formatting numebers like 1,000

    println!("my ID is {ID}");

    let (fname, lname, age2) = ("John", "Lennon", 25);

    println!("My name is {fname} {lname} and I was {age2}");
}