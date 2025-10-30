// nama : ItzYuuRz
/*
Program by ItzYuuRz 30/Sep
*/

use std::fmt::Display;

#[derive(Debug, Default)]
struct DataKontak<'a> {
    nama: &'a str,
    email: &'a str,
    telepon: u64,
}

impl Display for DataKontak<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "===========\n")?;
        write!(f, "Nama: [{}]\n", self.nama)?;
        write!(f, "Email: [{}]\n", self.email)?;
        write!(f, "Telepon: +62 {}\n", self.telepon)?;
        write!(f, "===========\n")
    }
}

fn main() {
    let kontak1 = DataKontak::default();
    println!("{:#?}", kontak1);
    println!("{}", kontak1);
}