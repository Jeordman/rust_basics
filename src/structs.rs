// trad struct
struct Color {
    red: u8,
    green: u8,
    blue: u8
}

// tuple struct
struct Color2(u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    // construct person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }

    fn full_name(&self) -> String {
        return format!("{} {}", self.first_name, self.last_name);
    }
}

pub fn run() {
    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0,
    };

    c.red = 3;

    println!("Color: {} {} {}", c.red, c.green, c.blue);

    let mut c2 = Color2(10, 30, 32);

    c2.0 = 0;

    println!("Color2: {} {} {}", c2.0, c2.1, c2.2);

    let mut p = Person::new("John", "Doe");
}
