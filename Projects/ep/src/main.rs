// nama : ItzYuuRz
/*
Program by ItzYuuRz 30/Sep
*/

#[derive(Debug, Default)]
struct DataKontak<'a> {
    nama: &'a str,
    email: &'a str,
    telepon: u64,
}


fn main() {
    let kontak1 = DataKontak::default();
    println!("{:#?}", kontak1);
}