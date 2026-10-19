mod cli;
mod db;
mod errors;
mod models;
mod tui;

use clap::Parser;
use cli::{Cli, Commands};
use db::Database;

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    let db = Database::new(&args.db_path)?;

    match args.command {
        Some(Commands::SinifEkle { ad }) => {
            let id = db.create_classroom(&ad)?;
            println!("✅ '{}' sınıfı başarıyla oluşturuldu (ID: {}).", ad, id);
        }

        Some(Commands::SinifListele) => {
            let classrooms = db.list_classrooms()?;
            if classrooms.is_empty() {
                println!("Henüz kayıtlı sınıf bulunmuyor.");
            } else {
                println!("{:<5} | {:<15}", "ID", "Sınıf Adı");
                println!("-----------------------");
                for c in classrooms {
                    println!("{:<5} | {:<15}", c.id, c.name);
                }
            }
        }

        Some(Commands::OgrenciEkle {
            no,
            ad,
            soyad,
            sinif_id,
        }) => {
            let id = db.create_student(&no, &ad, &soyad, sinif_id)?;
            println!(
                "✅ Öğrenci kaydedildi: [{}] {} {} (Kayıt ID: {}, Sınıf ID: {})",
                no, ad, soyad, id, sinif_id
            );
        }

        Some(Commands::SinifDegistir {
            ogrenci_no,
            yeni_sinif_id,
        }) => {
            db.transfer_student_classroom(&ogrenci_no, yeni_sinif_id)?;
            println!(
                "🔄 {} numaralı öğrencinin sınıfı (Yeni Sınıf ID: {}) olarak güncellendi.",
                ogrenci_no, yeni_sinif_id
            );
        }

        Some(Commands::OgrenciRapor { ogrenci_no }) => {
            let (student, history) = db.get_student_attendance_history(&ogrenci_no)?;
            println!("====================================================");
            println!(
                "ÖĞRENCİ KARTI: [{}] {} {} | Güncel Sınıfı: {} (ID: {})",
                student.student_number,
                student.first_name,
                student.last_name,
                student.classroom_name.unwrap_or_default(),
                student.classroom_id
            );
            println!("====================================================");

            if history.is_empty() {
                println!("Bu öğrenciye ait henüz yoklama kaydı bulunmuyor.");
            } else {
                // YENİ: "Kayıt No" kelimesini doğrudan format metninin içine aldık
                println!(
                    "{:<12} | {:<10} | {:<15} | Kayıt No",
                    "Tarih", "Durum", "Alındığı Sınıf"
                );
                println!("------------------------------------------------------------");
                for (att, recorded_class) in history {
                    println!(
                        "{:<12} | {:<10} | {:<15} | #{} (Sınıf ID: {})",
                        att.date,
                        att.status.to_string(),
                        recorded_class,
                        att.id,
                        att.recorded_classroom_id
                    );
                }
            }
        }

        // Argüman verilmediğinde veya "tui" yazıldığında arayüzü başlat!
        Some(Commands::Tui) | None => {
            tui::run_tui(db)?;
        }
    }

    Ok(())
}
