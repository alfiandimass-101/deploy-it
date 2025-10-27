use std::default;

#[derive(PartialEq, Eq)]
struct Water {
    ammount: usize,
}

type Air = Water;

impl Default for Water {
    fn default() -> Self {
        Water {
            ammount: 1,
        }
    }
}

impl std::ops::Add<Water> for Water {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        self.ammount += rhs.ammount;
        self
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", std::cmp::PartialEq::eq(&Water::default(), &Air::default()));
    Ok(())
}