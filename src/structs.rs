// mostly made to create custom data types

// traditional struct
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// tuple struct
struct TColor(u8,u8,u8);

struct Person {
    name: String,
    age: u8
}

impl Person {
    // Construct Person
    fn new(name: &str, age: i32) -> Person {
        Person {
            name: name.to_string(),
            age: age as u8
        }
    }
    
    fn name_and_age(&self) -> String {
        format!("{} {}", self.name, self.age)
    }

    fn set_age(&mut self, new_age: i32) {
        self.age = new_age as u8
    }
}

pub fn run() {
    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0
    };
    c.red = 200;
    println!("Color: {}, {}, {}", c.red, c.green, c.blue);

    let mut tc = TColor(255, 0, 0);
    tc.1 = 50;
    println!("Color: {}, {}, {}", tc.0, tc.1, tc.2);

    let mut p = Person::new("My Guy", 18);
    println!("Person {}", p.name_and_age());
    p.set_age(21);
    println!("Person {}", p.name_and_age());

}