use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Veritabanı hatası: {0}")]
    Database(#[from] rusqlite::Error),

    #[error("Terminal G/Ç hatası: {0}")]
    Io(#[from] std::io::Error),

    #[error("Geçersiz yoklama durumu: {0}")]
    InvalidStatus(String),

    #[error("Kayıt bulunamadı: {0}")]
    NotFound(String),
}
