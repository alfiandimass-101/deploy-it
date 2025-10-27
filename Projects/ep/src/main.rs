use std::default;

#[derive(PartialEq, Eq)]
struct Water {
    ammount: usize,
}

type Air = Water;

impl Default for Water {
    fn default() -> Self {
        Water {
            ammount: 0,
        }
    }
}

impl std::ops::Add for Water {
    fn add(self, rhs: Self) -> Self::Output {
        
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", std::cmp::PartialEq::eq(&Water::default(), &Air::default()));
    Ok(())
}