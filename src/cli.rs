use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "yoklama_sistemi",
    version = "0.1.0",
    about = "SQLite ve Ratatui tabanlı Öğrenci Yoklama Yönetim Sistemi"
)]
pub struct Cli {
    /// Veritabanı dosya yolu (varsayılan: yoklama.db)
    #[arg(short, long, default_value = "yoklama.db")]
    pub db_path: String,

    /// Alt komutlar (Boş bırakılırsa doğrudan Ratatui TUI ekranı açılır)
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Yeni bir sınıf/şube oluşturur (Örn: sinif-ekle --ad "10-A")
    SinifEkle {
        #[arg(short, long)]
        ad: String,
    },

    /// Kayıtlı tüm sınıfları listeler
    SinifListele,

    /// Yeni bir öğrenci kaydeder
    OgrenciEkle {
        #[arg(short, long)]
        no: String,
        #[arg(short, long)]
        ad: String,
        #[arg(short, long)]
        soyad: String,
        #[arg(short = 'c', long)]
        sinif_id: i64,
    },

    /// Öğrencinin sınıfını değiştirir (Devamsızlık geçmişi öğrenciyle taşınır)
    SinifDegistir {
        #[arg(short, long)]
        ogrenci_no: String,
        #[arg(short, long)]
        yeni_sinif_id: i64,
    },

    /// Bir öğrencinin tüm yoklama ve devamsızlık geçmişini raporlar
    OgrenciRapor {
        #[arg(short, long)]
        ogrenci_no: String,
    },

    /// Ratatui etkileşimli yoklama alma ekranını (TUI) başlatır
    Tui,
}
