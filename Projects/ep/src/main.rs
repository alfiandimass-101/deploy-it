use std::fs::File;
use std::io::Write;
use std::slice;

fn main() {
    // 1. Alokasikan variabel 'x' (dalam hal ini, sebuah Vec)
    // Kita alokasikan 100 byte.
    let data: Vec<u8> = (0..100).collect();
    let ptr = data.as_ptr();

    println!("Alamat memori 'data' dimulai di: {:?}", ptr);

    // 2. Ini adalah permintaan Anda: 20MB
    // 1 MB = 1024 * 1024 byte
    let size_to_read = 20 * 1024 * 1024; // 20 MB

    println!("PERINGATAN: Mencoba membaca {} byte dari pointer...", size_to_read);
    println!("Program ini hampir pasti akan CRASH (Segmentation Fault) sekarang.");
    println!("Kita hanya mengalokasikan 100 byte, tapi kita mencoba membaca 20,971,520 byte.");
    
    // 3. Blok unsafe untuk mencoba membaca memori
    // Ini adalah operasi yang sangat tidak aman dan akan gagal.
    // OS akan menghentikan program ini.
    let memory_slice = unsafe {
        // PERINTAH INI AKAN MENYEBABKAN SEGMENTATION FAULT
        slice::from_raw_parts(ptr, size_to_read)
    };

    // 4. Kode ini tidak akan pernah tercapai
    println!("(Keajaiban) Berhasil membaca memori! Menulis ke file...");
    
    // Jika program secara ajaib tidak crash, ia akan mencoba menulis ke file.
    match File::create("memory_dump.txt") {
        Ok(mut file) => {
            match file.write_all(memory_slice) {
                Ok(_) => println!("Berhasil menulis 20MB memori ke memory_dump.txt"),
                Err(e) => println!("Gagal menulis file: {}", e),
            }
        }
        Err(e) => println!("Gagal membuat file: {}", e),
    }

    println!("Selesai. (Anda seharusnya tidak melihat pesan ini)");
}
