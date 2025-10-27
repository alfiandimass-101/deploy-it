#[derive(PartialEq, Eq)]
struct Water;
type Air = Water;

impl Default for Water {
    fn default() -> Self {
        Water
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", std::cmp::PartialEq::eq(&Water::default(), &Air::default()));
    Ok(())
}