fn main() {
    let mut a = 10;
    let b = &mut a;
    println!("{}", b*&a);
}