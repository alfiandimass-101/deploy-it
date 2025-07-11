use burn::{backend::{Autodiff, Candle}, nn::{Linear, LinearConfig}, optim::{AdamConfig, GradientsParams, Optimizer}, prelude::*};

// --- 1. Definisikan Backend(Mesin Penghitung) kita ---
// kita akan menggunakan NdArray dengan elemen float f32 precision.
type MyBackend = Autodiff<Candle<f32>>;

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
        Self { linear: config.init(&Default::default()) }
    }

    // Fungsi forward pass: bagaimana data mengalir melalui model.
    pub fn forward(&self, data: Tensor<B, 2>) -> Tensor<B, 2> {
        self.linear.forward(data)
    }    
}

// --- 3. Proses Training
pub fn run() {
    // perangkat yang digunakan dalam training, ialah cpu.
    let device = Default::default();

    // inisialisasi model
    let mut model = Model::<MyBackend>::new();

    // Inisialisasi optimizer. Adam yang akan digunakan.
    // Optimizer inilah yang akan memperbarui weight model.
    let mut optm = AdamConfig::new().init();
    
    // --- Data sintesis ---
    // X adalah input, Y adalah output yang benar.
    // y = 2x + 1
    let x_train: Tensor<MyBackend, 2> = Tensor::from_floats([[1.0], [2.0], [3.0], [4.0]], &device).require_grad();
    let y_train: Tensor<MyBackend, 2> = Tensor::from_floats([[3.0], [5.0], [7.0], [9.0]], &device);

    // --- Loop training ---
    println!("Memulai training...");
    for epoch in 1..=60000 {
        // Forward pass: berikan input kepada model dan dapatkan sebuah prediksi.
        let output = model.forward(x_train.clone());

        // Hitung Loss: Seberapa jauh prediksi dari data benar.
        // Mean square error(MSE)
        let loss = (output - y_train.clone()).powf_scalar(2.0).mean(); // (prediksi - benar)^2

        if epoch % 1000 == 0 {
            println!("Epoch : {epoch}, Loss: {}", loss.to_data());
        }

        // Backward pass(Backpropagation) : hitung gradien (siapa yang salah? gatau).
        let gradi = GradientsParams::from_grads(loss.backward(), &model);
        // Optimizer step: perbarui weight model berdasarkan gradiens.
        model = optm.step(1e-2, model, gradi);
    }
     // --- 4. Inference (Menggunakan Model yang Sudah Dilatih) ---
     println!("\nTraining selesai! Gast tes.");
     let x_test: Tensor<MyBackend, 2> = Tensor::from_floats([[5.0]], &device);
     let y_pred = model.forward(x_test);
 
     println!("Input: 5.0, Prediksi Model: {}", y_pred.to_data());
     println!("Jawaban seharusna: {}", 2.0 * 5.0 + 1.0);
}

fn main() {
    run();
}
