// Traditional struct
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// Tuple struct
struct T_Color(u8, u8, u8);

// Practical struct, Person
struct Person {
    first_name: String,
    last_name: String,
    person_age: i32,
}

// Implementation of struct, Person
impl Person {
    // constructing a new "Person"
    fn new(first: &str, last: &str, age: i32) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
            person_age: age,
        }
    }

    // return only the name using '&self'
    // NO SEMICOLON at the end because we are RETURNING the value
    fn return_only_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    // change the last name using '&mut self'
    fn change_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    // tuple struct example
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

// PUBLIC FUNCTION
pub fn run() {
    /*let mut color = Color {
        red: 255,
        green: 0,
        blue: 0,
    };

    println!("Color = {} {} {}", color.red, color.green, color.blue);

    let mut tcolor = T_Color(0, 255, 0);

    println!("TColor = {} {} {}", tcolor.0, tcolor.1, tcolor.2);*/
    let mut person = Person::new("John", "Doe", 26);
    println!("First name: {}, Last name: {}, Age: {}", person.first_name, person.last_name, person.person_age);

    println!("Person's full name: {}", person.return_only_name());
    person.change_last_name("Mayer");
    println!("Changed last name to: {}", person.return_only_name());

    println!("Person tuple: {:?}", person.to_tuple());
}
