use crate::models::AttendanceStatus;
use crate::tui::app::App;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style}, // Stylize silindi
    text::Span,                      // Line silindi
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
};

pub fn draw(frame: &mut Frame, app: &mut App) {
    // Ekranı 3 dikey parçaya (Header, Tablo, Footer) bölelim
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Üst Başlık (3 satır)
            Constraint::Min(5),    // Tablo (Kalan tüm alan)
            Constraint::Length(3), // Alt Bilgi/Yardım (3 satır)
        ])
        .split(frame.area());

    // 1. ÜST BAŞLIK (HEADER)
    let class_name = app
        .classrooms
        .get(app.selected_class_idx)
        .map(|c| c.name.as_str())
        .unwrap_or("Sınıf Yok");

    let header_text = format!(
        " 🏫 YOKLAMA SİSTEMİ | Tarih: {} | Seçili Sınıf: {} (Sol/Sağ ok ile değiştir) ",
        app.today, class_name
    );
    let header = Paragraph::new(header_text)
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(header, chunks[0]);

    // 2. TABLO
    let header_cells = ["Öğrenci No", "Ad", "Soyad", "Durum"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::Yellow)));
    let header_row = Row::new(header_cells).height(1).bottom_margin(1);

    let rows = app.students.iter().map(|s| {
        let status_str = match s.status {
            Some(AttendanceStatus::Present) => {
                Span::styled("Geldi", Style::default().fg(Color::Green))
            }
            Some(AttendanceStatus::Absent) => Span::styled("Yok", Style::default().fg(Color::Red)),
            Some(AttendanceStatus::Excused) => {
                Span::styled("İzinli", Style::default().fg(Color::Blue))
            }
            Some(AttendanceStatus::Late) => Span::styled("Geç", Style::default().fg(Color::Yellow)),
            None => Span::styled("-", Style::default().fg(Color::DarkGray)),
        };

        Row::new(vec![
            Cell::from(s.student_number.clone()),
            Cell::from(s.first_name.clone()),
            Cell::from(s.last_name.clone()),
            Cell::from(status_str),
        ])
    });

    let table = Table::new(
        rows,
        [
            Constraint::Length(15),
            Constraint::Length(15),
            Constraint::Length(15),
            Constraint::Length(15),
        ],
    )
    .header(header_row)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Öğrenci Listesi "),
    )
    // ESKİ: .highlight_style(...)
    // YENİ:
    .row_highlight_style(Style::default().add_modifier(Modifier::REVERSED))
    .highlight_symbol(">> ");

    // Tablo stateful bir widget'tır (seçili satırı hatırlaması için app.table_state veriyoruz)
    frame.render_stateful_widget(table, chunks[1], &mut app.table_state);

    // 3. ALT BİLGİ (FOOTER)
    let footer = Paragraph::new(
        "Yön Tuşları: Gezinme | [G]: Geldi | [Y]: Yok | [İ]: İzinli | [C]: Geç | [Q/Esc]: Çıkış",
    )
    .block(Block::default().borders(Borders::ALL));
    frame.render_widget(footer, chunks[2]);
}
