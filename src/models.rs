use crate::errors::AppError;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttendanceStatus {
    Present,
    Absent,
    Excused,
    Late,
}

impl fmt::Display for AttendanceStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            AttendanceStatus::Present => "Present",
            AttendanceStatus::Absent => "Absent",
            AttendanceStatus::Excused => "Excused",
            AttendanceStatus::Late => "Late",
        };
        write!(f, "{}", label)
    }
}

impl FromStr for AttendanceStatus {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Present" => Ok(AttendanceStatus::Present),
            "Absent" => Ok(AttendanceStatus::Absent),
            "Excused" => Ok(AttendanceStatus::Excused),
            "Late" => Ok(AttendanceStatus::Late),
            _ => Err(AppError::InvalidStatus(s.to_string())),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Classroom {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct Student {
    pub id: i64,
    pub student_number: String,
    pub first_name: String,
    pub last_name: String,
    pub classroom_id: i64,
    pub classroom_name: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Attendance {
    pub id: i64,
    pub student_id: i64,
    pub recorded_classroom_id: i64,
    pub date: String,
    pub status: AttendanceStatus,
}

// Ratatui yoklama tablosunda her bir satırı temsil edecek birleştirilmiş yapı (JOIN sonucu)
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DailyAttendanceRow {
    pub student_id: i64,
    pub student_number: String,
    pub first_name: String,
    pub last_name: String,
    pub classroom_id: i64,
    pub status: Option<AttendanceStatus>, // O gün henüz yoklama alınmadıysa None gelir!
}
