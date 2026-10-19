use crate::db::Database;
use crate::errors::AppError;
use crate::models::{Attendance, AttendanceStatus, Classroom, DailyAttendanceRow, Student};
use rusqlite::params;
use std::str::FromStr;

impl Database {
    // ==========================================
    // 1. SINIF (CLASSROOM) İŞLEMLERİ
    // ==========================================

    pub fn create_classroom(&self, name: &str) -> Result<i64, AppError> {
        self.conn
            .execute("INSERT INTO classroom (name) VALUES (?1)", params![name])?;
        Ok(self.conn.last_insert_rowid())
    }

    pub fn list_classrooms(&self) -> Result<Vec<Classroom>, AppError> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, name FROM classroom ORDER BY name ASC")?;
        let rows = stmt.query_map([], |row| {
            Ok(Classroom {
                id: row.get(0)?,
                name: row.get(1)?,
            })
        })?;

        let mut classrooms = Vec::new();
        for row in rows {
            classrooms.push(row?);
        }
        Ok(classrooms)
    }

    // ==========================================
    // 2. ÖĞRENCİ (STUDENT) İŞLEMLERİ
    // ==========================================

    pub fn create_student(
        &self,
        student_number: &str,
        first_name: &str,
        last_name: &str,
        classroom_id: i64,
    ) -> Result<i64, AppError> {
        self.conn.execute(
            "INSERT INTO student (student_number, first_name, last_name, classroom_id)
             VALUES (?1, ?2, ?3, ?4)",
            params![student_number, first_name, last_name, classroom_id],
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    /// Öğrencinin sınıfını değiştirir. Yoklama kayıtları student_id'ye bağlı olduğu için
    /// geçmiş devamsızlıklar kaybolmaz, öğrenciyle birlikte yeni sınıfa taşınır.
    pub fn transfer_student_classroom(
        &self,
        student_number: &str,
        new_classroom_id: i64,
    ) -> Result<(), AppError> {
        let affected = self.conn.execute(
            "UPDATE student SET classroom_id = ?1 WHERE student_number = ?2",
            params![new_classroom_id, student_number],
        )?;

        if affected == 0 {
            return Err(AppError::NotFound(format!(
                "Öğrenci numarası bulunamadı: {}",
                student_number
            )));
        }
        Ok(())
    }

    // ==========================================
    // 3. GÜNLÜK YOKLAMA (ATTENDANCE) İŞLEMLERİ
    // ==========================================

    /// Günlük yoklama kaydı ekler veya o gün zaten kayıt varsa günceller (UPSERT).
    pub fn upsert_attendance(
        &self,
        student_id: i64,
        recorded_classroom_id: i64,
        date: &str,
        status: AttendanceStatus,
    ) -> Result<(), AppError> {
        self.conn.execute(
            "INSERT INTO attendance (student_id, recorded_classroom_id, date, status)
             VALUES (?1, ?2, ?3, ?4)
             ON CONFLICT(student_id, date) DO UPDATE SET
                status = excluded.status,
                recorded_classroom_id = excluded.recorded_classroom_id",
            params![student_id, recorded_classroom_id, date, status.to_string()],
        )?;
        Ok(())
    }

    /// Seçili sınıftaki tüm öğrencileri ve (varsa) seçili gündeki yoklama durumlarını getirir.
    /// LEFT JOIN kullanıyoruz ki o gün henüz yoklaması alınmamış öğrenciler de listelensin.
    pub fn get_classroom_daily_attendance(
        &self,
        classroom_id: i64,
        date: &str,
    ) -> Result<Vec<DailyAttendanceRow>, AppError> {
        let mut stmt = self.conn.prepare(
            "SELECT s.id, s.student_number, s.first_name, s.last_name, s.classroom_id, a.status
             FROM student s
             LEFT JOIN attendance a ON s.id = a.student_id AND a.date = ?1
             WHERE s.classroom_id = ?2
             ORDER BY s.student_number ASC",
        )?;

        let rows = stmt.query_map(params![date, classroom_id], |row| {
            let status_str: Option<String> = row.get(5)?;
            let parsed_status = match status_str {
                Some(ref s) => AttendanceStatus::from_str(s).ok(),
                None => None,
            };

            Ok(DailyAttendanceRow {
                student_id: row.get(0)?,
                student_number: row.get(1)?,
                first_name: row.get(2)?,
                last_name: row.get(3)?,
                classroom_id: row.get(4)?,
                status: parsed_status,
            })
        })?;

        let mut result = Vec::new();
        for row in rows {
            result.push(row?);
        }
        Ok(result)
    }

    /// Bir öğrencinin tüm yoklama geçmişini (eski sınıflarında aldığı yoklamalar dahil!) getirir.
    pub fn get_student_attendance_history(
        &self,
        student_number: &str,
    ) -> Result<(Student, Vec<(Attendance, String)>), AppError> {
        // 1. Önce öğrenciyi ve güncel sınıf adını bulalım
        let student = self
            .conn
            .query_row(
                "SELECT s.id, s.student_number, s.first_name, s.last_name, s.classroom_id, c.name
             FROM student s
             JOIN classroom c ON s.classroom_id = c.id
             WHERE s.student_number = ?1",
                params![student_number],
                |row| {
                    Ok(Student {
                        id: row.get(0)?,
                        student_number: row.get(1)?,
                        first_name: row.get(2)?,
                        last_name: row.get(3)?,
                        classroom_id: row.get(4)?,
                        classroom_name: Some(row.get(5)?),
                    })
                },
            )
            .map_err(|_| AppError::NotFound(format!("Öğrenci bulunamadı: {}", student_number)))?;

        // 2. Öğrencinin tüm yoklama geçmişini (o tarihte hangi sınıfta olduğu bilgisiyle) çekelim
        let mut stmt = self.conn.prepare(
            "SELECT a.id, a.student_id, a.recorded_classroom_id, a.date, a.status, c.name
             FROM attendance a
             JOIN classroom c ON a.recorded_classroom_id = c.id
             WHERE a.student_id = ?1
             ORDER BY a.date DESC",
        )?;

        let rows = stmt.query_map(params![student.id], |row| {
            let status_str: String = row.get(4)?;
            let status =
                AttendanceStatus::from_str(&status_str).unwrap_or(AttendanceStatus::Present);
            let recorded_class_name: String = row.get(5)?;

            Ok((
                Attendance {
                    id: row.get(0)?,
                    student_id: row.get(1)?,
                    recorded_classroom_id: row.get(2)?,
                    date: row.get(3)?,
                    status,
                },
                recorded_class_name,
            ))
        })?;

        let mut history = Vec::new();
        for row in rows {
            history.push(row?);
        }

        Ok((student, history))
    }
}
