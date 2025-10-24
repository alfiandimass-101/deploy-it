#[derive(Debug, Default)]
struct Human<'a> {
    name: &'a str,
    age: u32,
}

impl<'a> Human<'a> {
    fn set_name(&mut self, name: &'a str) {
        self.name = name;
    }
}

fn main() {
    let human1 = Human::default();
    
}
