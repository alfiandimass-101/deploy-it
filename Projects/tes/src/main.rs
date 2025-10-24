fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut i = 0u128;
    loop {
        i += 1;
        if i == 1_000_000_000_000 {
            break;
        }
    }
    Ok(())
}