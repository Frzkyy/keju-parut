use std::io::{self, Write};
use std::collections::HashMap;
fn main() {
    let mut nilai_mahasiswa: HashMap<String,i32> = HashMap::new();
    let mut app_jalan: bool = true;
    while app_jalan {
        let mut nama = String::new();
        let mut nilai_str = String::new();

        print!("Masukkan Nama Siswa: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut nama).unwrap();
        print!("Masukkan Nilai: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut nilai_str).unwrap();

        let nilai: i32 = nilai_str.trim().parse().unwrap();
        nilai_mahasiswa.insert(nama.trim().to_string(),nilai);


        let mut keluar:bool = true;
        
        while keluar {
            let mut lagi = String::new();
            print!("Apakah mau input lagi? y/n: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut lagi).unwrap();
            if  lagi.trim() == "y" || lagi.trim() == "Y" {
                keluar = false;
            } else if lagi.trim() == "n" || lagi.trim() == "N" {
                keluar = false;
                app_jalan = false;
            }else {
                println!("[Error] Masukkan hanya y/n");
            }
        }
    }

    for (nama,nilai) in nilai_mahasiswa {
        println!("Nama: {} | Nilai: {}", nama,nilai);
    }
}
