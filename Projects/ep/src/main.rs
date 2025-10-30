// Fungsi yang menerima closure dan memanggilnya.
// Closure harus mengimplementasikan trait `FnOnce` (dipanggil sekali).
fn apply<F>(f: F) where
    F: FnOnce() {
    f();
}

// Fungsi yang menerima closure dan mengembalikan i32.
// Closure harus mengimplementasikan trait `Fn` (dapat dipanggil berulang kali).
fn apply_to_3<F>(f: F) -> i32 where
    F: Fn(i32) -> i32 {
    f(3)
}

fn main() {
    let greeting = "hello".to_owned();
    let mut farewell = "goodbye".to_owned();

    // Contoh closure yang mengimplementasikan `FnOnce` karena `mem::drop`
    // memaksa `farewell` dipindahkan ke dalam closure.
    let diary = || {
        println!("Saya bilang {}", greeting);
        farewell.push_str("!");
        println!("Kemudian saya bilang {}", farewell);
        std::mem::drop(farewell); // Memaksa `farewell` dipindahkan
    };

    apply(diary);

    // Contoh closure yang mengimplementasikan `Fn`
    let double = |x| x * 2;
    println!("3 digandakan: {}", apply_to_3(double));
}
