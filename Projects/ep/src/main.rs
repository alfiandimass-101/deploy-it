#![feature(dropck_eyepatch)]

use std::marker::PhantomData;

struct ReferensiMasaHidupKhusus<'a, T>(&'a T, PhantomData<&'a ()>);

// Atribut `#[may_dangle]` secara khusus memberi tahu kompilator 
// bahwa tipe T (data yang dipinjam) bisa dijatuhkan sebelum ReferensiMasaHidupKhusus
// tanpa memicu kesalahan lifetime.
unsafe impl<'a, #[may_dangle] T> Drop for ReferensiMasaHidupKhusus<'a, T> {
    fn drop(&mut self) {
        println!("Referensi khusus dijatuhkan. Data T mungkin sudah tidak ada. (Ini diizinkan oleh #[may_dangle])");
    }
}

fn buat_dan_hancurkan_s() -> ReferensiMasaHidupKhusus<'static, String> {
    let referensi_gantung: ReferensiMasaHidupKhusus<'static, String>;

    { // Cakupan (Scope) sementara
        let s = String::from("data yang akan hilang");
        
        // Pinjam `s` dan secara paksa memberikannya 'static lifetime (TIDAK AMAN)
        // Note: Penggunaan `transmute` adalah operasi C-Style UNSAFE!
        // Ini adalah cara untuk memanipulasi lifetime untuk menciptakan referensi gantung.
        let pinjaman: &String = &s;
        let pinjaman_static = unsafe { std::mem::transmute::<&String, &'static String>(pinjaman) };

        referensi_gantung = ReferensiMasaHidupKhusus(pinjaman_static, PhantomData);
        
        // Ketika cakupan ini berakhir, `s` akan dijatuhkan secara OTOMATIS.
    } // <-- `s` dijatuhkan di sini!

    println!("Data `s` telah dihancurkan, tetapi `referensi_gantung` masih ada.");

    referensi_gantung
}

fn main() {
    let referensi = buat_dan_hancurkan_s();

    // WARNING: Jangan mencoba mengakses referensi.0. Ini akan menyebabkan UB (Undefined Behavior).
    // Bahkan meskipun kompiler mengizinkan struktur ini untuk dijatuhkan lebih akhir
    // karena adanya #[may_dangle], data yang ditunjuknya sudah tidak ada.
    
    println!("Menjatuhkan `referensi`...");
    println!("{}", referensi.0);
    drop(referensi); 
    println!("Selesai.");
}