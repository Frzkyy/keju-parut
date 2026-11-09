use std::collections::HashMap;
use std::io::{self, Write};

// Fungsi bantu: tampilkan pesan, baca ketikan, kembalikan teks bersih
fn baca_input(pesan: &str) -> String {
    print!("{}", pesan);
    io::stdout().flush().unwrap();

    let mut teks = String::new();
    io::stdin().read_line(&mut teks).unwrap();
    teks.trim().to_string()
}

// Fungsi bantu: baca angka, kalau salah kembalikan None
fn baca_nilai() -> Option<i32> {
    let teks = baca_input("Masukkan nilai: ");
    match teks.parse() {
        Ok(n) => Some(n),
        Err(_) => {
            println!("[Error] Nilai harus angka");
            None
        }
    }
}

fn main() {
    let mut data: HashMap<String, i32> = HashMap::new();

    loop {
        println!("\n=== MENU ===");
        println!("1. Tambah data (Create)");
        println!("2. Lihat data (Read)");
        println!("3. Ubah nilai (Update)");
        println!("4. Hapus data (Delete)");
        println!("5. Keluar");

        let pilihan = baca_input("Pilih: ");

        match pilihan.as_str() {
            "1" => {
                let nama = baca_input("Masukkan nama: ");
                if data.contains_key(&nama) {
                    println!("[Error] Nama sudah ada, pakai menu Ubah");
                    continue;
                }
                if let Some(nilai) = baca_nilai() {
                    data.insert(nama, nilai);
                    println!("Data tersimpan");
                }
            }
            "2" => {
                if data.is_empty() {
                    println!("Belum ada data");
                } else {
                    for (nama, nilai) in &data {
                        println!("Nama: {} | Nilai: {}", nama, nilai);
                    }
                }
            }
            "3" => {
                let nama = baca_input("Nama yang mau diubah: ");
                if data.contains_key(&nama) {
                    if let Some(nilai) = baca_nilai() {
                        data.insert(nama, nilai);
                        println!("Data diubah");
                    }
                } else {
                    println!("[Error] Nama tidak ditemukan");
                }
            }
            "4" => {
                let nama = baca_input("Nama yang mau dihapus: ");
                match data.remove(&nama) {
                    Some(_) => println!("Data dihapus"),
                    None => println!("[Error] Nama tidak ditemukan"),
                }
            }
            "5" => {
                println!("Sampai jumpa!");
                break;
            }
            _ => println!("[Error] Pilihan tidak valid"),
        }
    }
}