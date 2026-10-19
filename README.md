# 🏫 Yoklama Sistemi

Rust, SQLite ve Ratatui kullanılarak geliştirilmiş; ultra hızlı, taşınabilir ve etkileşimli terminal arayüzüne (TUI) sahip profesyonel bir Öğrenci Yoklama Yönetim Sistemidir.

Sistem, "Domain-Driven Design" prensipleriyle tasarlanmıştır: Öğrenci sınıf değiştirdiğinde bile yoklama geçmişi ve eski devamsızlık kayıtları öğrenciyle birlikte kalır.

## 🚀 Özellikler

* **Çift Arayüz:** Hem CLI (Komut Satırı) hem de TUI (Tam Ekran Terminal) desteği.
* **Akıllı Veritabanı:** `yoklama.db` adında yerleşik, taşınabilir SQLite veritabanı.
* **Upsert Mantığı:** Aynı öğrenciye aynı gün içinde mükerrer kayıt girilmesini veritabanı düzeyinde engeller.
* **Gelişmiş Raporlama:** Öğrencilerin eski sınıflarındaki devamsızlıklarını bile tarihçesiyle raporlar.

---

## 🛠️ Kurulum ve Çalıştırma

Bu program hiçbir dış bağımlılık veya veritabanı sunucusu kurulumu gerektirmez. Sadece çalıştırılabilir dosyayı indirip çalıştırmanız yeterlidir.

* **Windows:** `yoklama_sistemi-windows.exe` dosyasını indirin. Komut satırında veya PowerShell'de `.\yoklama_sistemi-windows.exe` yazarak çalıştırın.
* **macOS / Linux:** İlgili dosyayı indirin, terminalden çalıştırılabilir izni verin (`chmod +x yoklama_sistemi-macos`) ve `./yoklama_sistemi-macos` yazarak çalıştırın.

---

## 📚 Hızlı Başlangıç (Veritabanını Doldurma)

Programı ilk indirdiğinizde `yoklama.db` veritabanı boştur. Etkileşimli yoklama ekranını kullanabilmek için önce CLI komutlarıyla sınıfları ve öğrencileri sisteme eklemeniz gerekir.

*(Not: Aşağıdaki örnekler Windows içindir. macOS/Linux kullanıyorsanız `.exe` kısmını kendi dosya adınıza göre uyarlayın.)*

### 1. Sınıfları Oluşturun

Önce okuldaki şubeleri tanımlıyoruz:
```bash
yoklama_sistemi.exe sinif-ekle --ad "10-A"
yoklama_sistemi.exe sinif-ekle --ad "10-B"
```
Oluşturulan sınıfları ve ID numaralarını listelemek için:

`yoklama_sistemi.exe sinif-listele`

### 2. Öğrencileri Kaydedin

```bash
yoklama_sistemi.exe ogrenci-ekle --no "1001" --ad "Ahmet" --soyad "Yılmaz" --sinif-id 1
yoklama_sistemi.exe ogrenci-ekle --no "1002" --ad "Zeynep" --soyad "Kaya" --sinif-id 1
yoklama_sistemi.exe ogrenci-ekle --no "1003" --ad "Mehmet" --soyad "Demir" --sinif-id 2
```

### 3. Etkileşimli Yoklama Ekranını (TUI) Başlatın

```bash
yoklama_sistemi.exe
```

### 🔄 Gelişmiş İşlemler

Öğrencinin Sınıfını Değiştirmek
```bash
yoklama_sistemi.exe sinif-degistir --ogrenci-no "1001" --yeni-sinif-id 2
```

Öğrencinin devamsızlık raporunu almak
```bash
yoklama_sistemi.exe ogrenci-rapor --ogrenci-no "1001"
```

### 👨‍💻 Geliştiriciler İçin (Kaynaktan Derleme)

```bash
git clone [https://github.com/ozgurturkiye/yoklama_sistemi.git](https://github.com/ozgurturkiye/yoklama_sistemi.git)
cd yoklama_sistemi
cargo run             # Geliştirme modunda çalıştırmak için
cargo build --release # Üretim/Canlı sürümünü derlemek için
```




