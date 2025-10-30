// Mengembalikan sebuah closure menggunakan `impl Fn`.
// Ini adalah cara yang direkomendasikan untuk mengembalikan closure.
fn create_adder(a: i32) -> impl Fn(i32) -> i32 {
    // Kata kunci `move` diperlukan jika kita menangkap variabel dari lingkungan
    move |b| a + b
}

fn main() {
    let add_five = create_adder(5);
    println!("5 + 3 = {}", add_five(3)); // Output: 5 + 3 = 8
}
