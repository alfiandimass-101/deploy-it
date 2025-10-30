// nama : ItzYuuRz
/*
Program by ItzYuuRz 30/Sep
*/

use std::fmt::{Display, write};

#[derive(Debug, Default)]
struct DataKontak<'a> {
    nama: &'a str,
    email: &'a str,
    telepon: u64,
}

impl Display for DataKontak {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write(f, "===========")?;
        write(f, "Nama: [{}]", self.nama)?;
        
    }
}

fn main() {
    let kontak1 = DataKontak::default();
    println!("{:#?}", kontak1);
}