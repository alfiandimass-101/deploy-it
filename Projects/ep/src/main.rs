use std::fs::File;
use std::io::Write;
use std::ptr; // Kita perlu modul pointer

fn main() {
    // 1. Alokasikan 100 byte.
    let data: Vec<u8> = (0..100).collect();
    let ptr = data.as_ptr();

    println!("Mem addr: {:?}", ptr);

    // 2. Target 20 MB
    let size_to_read = 20 * 1024 * 1024; // 20 MB
    
    // Kita siapkan buffer untuk menampung data (jika berhasil)
    let mut buffer: Vec<u8> = Vec::with_capacity(size_to_read);

    // println!("PERINGATAN: 'Segala cara' dimulai.");
    // println!("Mencoba membaca {} byte, SATU PER SATU dengan pointer...", size_to_read);
    // println!("Ini akan 100% CRASH dengan 'Segmentation Fault' (SIGSEGV) setelah byte ke-100.");
    
    // 3. Blok unsafe - "Segala Cara"
    unsafe {
        for i in 0..size_to_read {
            // Kita hitung alamat pointer berikutnya
            let current_ptr = ptr.add(i);

            // ptr::read_volatile memaksa CPU membaca dari alamat memori ini
            // DAN melarang compiler untuk mengoptimasi pembacaan ini.
            //
            // Ini akan berhasil untuk i = 0 sampai 99.
            // Saat i = 100 (atau di batas 'page' memori berikutnya),
            // CPU akan mencoba membaca alamat yang tidak terdaftar (unmapped).
            // OS akan mendeteksi ini dan mengirim sinyal SIGSEGV.
            // Program akan dihentikan paksa oleh OS.
            let byte = ptr::read_volatile(current_ptr);
            
            // Jika berhasil dibaca, masukkan ke buffer
            buffer.push(byte);
        }
    }

    // 4. KODE INI TIDAK AKAN PERNAH TERCAPAI
    println!("(MUSTAHIL) Berhasil membaca memori! Menulis ke file...");
    
    match File::create("memory_dump.txt") {
        Ok(mut file) => {
            match file.write_all(&buffer) {
                Ok(_) => println!("Berhasil menulis 20MB memori ke memory_dump.txt"),
                Err(e) => println!("Gagal menulis file: {}", e),
            }
        }
        Err(e) => println!("Gagal membuat file: {}", e),
    }

    println!("Selesai. (Anda seharusnya tidak melihat pesan ini)");
}

