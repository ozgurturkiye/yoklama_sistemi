use crate::{
    db::Database,
    models::{AttendanceStatus, Classroom, DailyAttendanceRow},
};
use chrono::Local;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{DefaultTerminal, widgets::TableState};

pub struct App {
    pub db: Database,
    pub should_quit: bool,
    pub classrooms: Vec<Classroom>,
    pub selected_class_idx: usize,
    pub students: Vec<DailyAttendanceRow>,
    pub table_state: TableState,
    pub today: String,
}

impl App {
    pub fn new(db: Database) -> Self {
        // Bugünün tarihini (YYYY-MM-DD) al
        let today = Local::now().format("%Y-%m-%d").to_string();
        let classrooms = db.list_classrooms().unwrap_or_default();

        let mut app = Self {
            db,
            should_quit: false,
            classrooms,
            selected_class_idx: 0,
            students: Vec::new(),
            table_state: TableState::default(),
            today,
        };
        app.load_students();
        app
    }

    /// Seçili sınıfın öğrencilerini veritabanından çeker
    pub fn load_students(&mut self) {
        if let Some(class) = self.classrooms.get(self.selected_class_idx) {
            self.students = self
                .db
                .get_classroom_daily_attendance(class.id, &self.today)
                .unwrap_or_default();

            // Eğer liste doluysa ve tabloda hiçbir satır seçili değilse ilk satırı seç
            if !self.students.is_empty() && self.table_state.selected().is_none() {
                self.table_state.select(Some(0));
            }
        }
    }

    /// Saniyede 60 kare çalışan (60 FPS) ana oyun/arayüz döngüsü
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> anyhow::Result<()> {
        while !self.should_quit {
            terminal.draw(|frame| crate::tui::ui::draw(frame, self))?;

            // YENİ: İç içe 3 if bloğunu tek satırda birleştirdik
            if event::poll(std::time::Duration::from_millis(16))?
                && let Event::Key(key) = event::read()?
                && key.kind == KeyEventKind::Press
            {
                self.handle_key(key.code)?;
            }
        }
        Ok(())
    }

    fn handle_key(&mut self, key: KeyCode) -> anyhow::Result<()> {
        match key {
            KeyCode::Char('q') | KeyCode::Esc => self.should_quit = true,
            KeyCode::Down => self.next_student(),
            KeyCode::Up => self.previous_student(),
            KeyCode::Right => self.next_class(),
            KeyCode::Left => self.previous_class(),
            // Yoklama Kısayolları (G: Geldi, Y: Yok, I: İzinli, C: Geç)
            KeyCode::Char('g') | KeyCode::Char('G') => {
                self.mark_attendance(AttendanceStatus::Present)?
            }
            KeyCode::Char('y') | KeyCode::Char('Y') => {
                self.mark_attendance(AttendanceStatus::Absent)?
            }
            KeyCode::Char('i') | KeyCode::Char('I') => {
                self.mark_attendance(AttendanceStatus::Excused)?
            }
            KeyCode::Char('c') | KeyCode::Char('C') => {
                self.mark_attendance(AttendanceStatus::Late)?
            }
            _ => {}
        }
        Ok(())
    }

    fn next_student(&mut self) {
        let i = match self.table_state.selected() {
            Some(i) => {
                if i >= self.students.len().saturating_sub(1) {
                    i
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
    }

    fn previous_student(&mut self) {
        let i = match self.table_state.selected() {
            Some(i) => {
                if i == 0 {
                    0
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
    }

    fn next_class(&mut self) {
        if self.selected_class_idx + 1 < self.classrooms.len() {
            self.selected_class_idx += 1;
            self.load_students();
            self.table_state.select(Some(0));
        }
    }

    fn previous_class(&mut self) {
        if self.selected_class_idx > 0 {
            self.selected_class_idx -= 1;
            self.load_students();
            self.table_state.select(Some(0));
        }
    }

    fn mark_attendance(&mut self, status: AttendanceStatus) -> anyhow::Result<()> {
        // YENİ: Tablo seçimi, öğrenci verisi ve sınıf verisini tek if satırında bağladık
        if let Some(i) = self.table_state.selected()
            && let Some(student) = self.students.get(i)
            && let Some(class) = self.classrooms.get(self.selected_class_idx)
        {
            self.db
                .upsert_attendance(student.student_id, class.id, &self.today, status)?;
            self.load_students();
        }
        Ok(())
    }
}
