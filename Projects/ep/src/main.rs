struct Water;
type Air = Water;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", std::cmp::PartialEq::eq(&Water, &Air));
    Ok(())
}