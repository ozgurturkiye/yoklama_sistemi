pub mod app;
pub mod ui;

use crate::db::Database;
use app::App;

/// Terminali "Raw" moda alır, alternatif ekrana geçer ve TUI'yi başlatır.
/// İşlem bitince (hata alsa bile) terminali eski standart haline (restore) döndürür.
pub fn run_tui(db: Database) -> anyhow::Result<()> {
    let mut terminal = ratatui::init();

    // Uygulama durumunu başlat ve ana döngüyü çalıştır
    let mut app = App::new(db);
    let result = app.run(&mut terminal);

    // Uygulama kapandığında terminali bozmamak için mutlaka geri yükle
    ratatui::restore();

    result
}
