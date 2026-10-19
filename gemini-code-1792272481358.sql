-- 1. Sınıf / Şube Tablosu (Örn: 10-A, 11-B)
CREATE TABLE IF NOT EXISTS classroom (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE
);

-- 2. Öğrenci Tablosu
CREATE TABLE IF NOT EXISTS student (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    student_number TEXT NOT NULL UNIQUE,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    classroom_id INTEGER NOT NULL,
    FOREIGN KEY (classroom_id) REFERENCES classroom(id) ON DELETE RESTRICT
);

-- 3. Günlük Yoklama Tablosu (Doğrudan öğrenciye bağlı ve gün bazlı tekil)
CREATE TABLE IF NOT EXISTS attendance (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    student_id INTEGER NOT NULL,
    recorded_classroom_id INTEGER NOT NULL, -- Yoklamanın alındığı gün öğrencinin bulunduğu sınıf
    date TEXT NOT NULL,                     -- YYYY-MM-DD formatında gün bilgisi
    status TEXT NOT NULL,                   -- 'Present', 'Absent', 'Excused', 'Late'
    FOREIGN KEY (student_id) REFERENCES student(id) ON DELETE CASCADE,
    FOREIGN KEY (recorded_classroom_id) REFERENCES classroom(id) ON DELETE RESTRICT,
    UNIQUE(student_id, date)                -- Bir öğrencinin bir günde sadece 1 yoklama kaydı olabilir!
);