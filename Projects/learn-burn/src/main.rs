use burn::{backend::NdArray, nn::{Linear, LinearConfig}, prelude::*};


// --- 1. Definisikan Backend(Mesin Penghitung) kita ---
// kita akan menggunakan NdArray dengan elemen float f32 precision.
type MyBackend = NdArray<f32>;

// --- 2. Definisikan Model Kita ---
// Ini adalah otak dari mesin kita yang dibuat dengan sebuah struct.
// Anotasi derive dibawah ini meng-generate banyak kode boilerplate untuk kita.
#[derive(Debug, Module)]
pub struct Model<B: Backend> {
    // Model kita hanya punya satu layer, yaitu layer linear.
    // layer ini mencoba mempelajari hubungan y = Wx + b.
    linear: Linear<B>,
}

// Implementasi untuk model kita.
impl<B: Backend> Model<B> {
    // Fungsi untuk membuat model baru.
    // LinearConfig(1, 1) berarti: 1 input fitur (x) -> 1 output fitur (y).
    pub fn new() -> Self {
        let config = LinearConfig::new(1, 1);
        Self { linear: config.init(&B::Device) }
    }
}

fn main() {
    println!("Hello, world!");
}
