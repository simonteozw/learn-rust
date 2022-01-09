struct Person {
    first_name: String,
    last_name: String
}

impl Person {
    fn new(f: &str, l: &str) -> Person {
        Person {
            first_name: f.to_string(),
            last_name: l.to_string()
        }
    }

    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}


fn main() {
    let p = Person::new("John", "Smith");
    println!("full name {}", p.full_name());
}