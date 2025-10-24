use std::ffi::os_str::Display;

#[derive(Debug, Default)]
struct Human<'a> {
    name: &'a str,
    age: u32,
}

impl<'a> Human<'a> {
    fn set_name(&mut self, name: &'a str) {
        self.name = name;
    }

    fn set_age(&mut self, age: u32) {
        self.age = age;
    }

    fn desonstruct(self) -> (&'a str, u32) {
        (self.name, self.age)
    }
}

impl std::fmt::Display for Human<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", format!("name: {self.name}\nage: {self.age}"))
    }
}

fn main() {
    let mut human1 = Human::default();
    human1.set_name("apa ya");
    human1.set_age(16);
    println!("{:?}", human1);
    let (nm, ag) = human1.desonstruct();
    human1.set_name("aa");
}
