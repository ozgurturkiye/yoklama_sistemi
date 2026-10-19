pub mod repo;

use crate::errors::AppError;
use rusqlite::Connection;

pub struct Database {
    pub conn: Connection,
}

impl Database {
    /// Veritabanı dosyasını açar (yoksa oluşturur) ve tabloları hazırlar.
    pub fn new(db_path: &str) -> Result<Self, AppError> {
        let conn = Connection::open(db_path)?;

        // Kritik: SQLite'ta Foreign Key desteğini aktif et.
        conn.execute_batch("PRAGMA foreign_keys = ON;")?;

        let db = Self { conn };
        db.init_tables()?;
        Ok(db)
    }

    /// Tekil isimlendirme standardımıza uygun 3 çekirdek tablomuzu oluşturur.
    fn init_tables(&self) -> Result<(), AppError> {
        self.conn.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS classroom (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL UNIQUE
            );

            CREATE TABLE IF NOT EXISTS student (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                student_number TEXT NOT NULL UNIQUE,
                first_name TEXT NOT NULL,
                last_name TEXT NOT NULL,
                classroom_id INTEGER NOT NULL,
                FOREIGN KEY (classroom_id) REFERENCES classroom(id) ON DELETE RESTRICT
            );

            CREATE TABLE IF NOT EXISTS attendance (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                student_id INTEGER NOT NULL,
                recorded_classroom_id INTEGER NOT NULL,
                date TEXT NOT NULL,
                status TEXT NOT NULL,
                FOREIGN KEY (student_id) REFERENCES student(id) ON DELETE CASCADE,
                FOREIGN KEY (recorded_classroom_id) REFERENCES classroom(id) ON DELETE RESTRICT,
                UNIQUE(student_id, date)
            );
            ",
        )?;
        Ok(())
    }
}
