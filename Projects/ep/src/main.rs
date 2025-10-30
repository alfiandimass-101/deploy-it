fn main() {
    let mut a = 10;
    let b = &a;
    println!("{}", *b*&*a);
}