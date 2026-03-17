# Uygulama Planı: System Update Manager

## Genel Bakış

Bu uygulama planı, Tauri 2.0 tabanlı çapraz platform (Windows, macOS, Linux) sistem güncelleme yöneticisinin adım adım geliştirilmesini içerir. Her görev, önceki görevler üzerine inşa edilir ve tüm bileşenler sonunda entegre edilir.

## Görevler

- [x] 1. Proje yapısını ve temel altyapıyı kur
  - Tauri 2.0 projesi oluştur (Rust backend + React/TypeScript frontend)
  - Proje dizin yapısını organize et (src/backend, src/frontend, src/adapters)
  - Gerekli bağımlılıkları ekle (tokio, serde, sqlx, proptest)
  - Frontend için state management kur (Zustand veya Redux)
  - Temel Tauri 2.0 yapılandırmasını ayarla (tauri.conf.json)
  - _Gereksinimler: 11.1, 11.4, 13.1_

- [-] 2. Platform tespit ve adaptör altyapısını implement et
  - [x] 2.1 OS_Detector bileşenini implement et
    - Platform tespiti (Windows/macOS/Linux)
    - Versiyon tespiti (Windows 10/11, macOS 12+, Linux dağıtımları)
    - /etc/os-release parser (Linux için)
    - _Gereksinimler: 1.1, 10.2, 11.2_
  
  - [-] 2.2 OS_Detector için property testi yaz
    - **Özellik 1: Platform Tespiti**
    - **Özellik 25: Platform Tespit Doğruluğu**
    - **Doğrular: Gereksinim 1.1, 10.2, 11.2**
  
  - [ ] 2.3 PlatformAdapter trait'ini tanımla
    - Tüm platform-spesifik metodları içeren trait
    - Paket yöneticisi, sürücü ve yetki yönetimi metodları
    - _Gereksinimler: 10.3, 10.6_
  
  - [ ] 2.4 Platform enum'larını ve veri modellerini oluştur
    - Platform, WindowsVersion, MacOSVersion, LinuxDistro
    - PackageManager enum (Winget, Chocolatey, Homebrew, mas-cli, apt, dnf, pacman, Flatpak, Snap)
    - _Gereksinimler: 10.1, 11.1_

- [ ] 3. Checkpoint - Temel altyapı testi
  - Tüm testlerin geçtiğinden emin ol, kullanıcıya sorular varsa sor.

- [ ] 4. Windows Adapter'ı implement et
  - [ ] 4.1 WindowsAdapter struct ve temel metodları
    - WingetClient ve ChocolateyClient entegrasyonu
    - WindowsUpdateClient için temel yapı
    - get_package_managers() implementasyonu
    - _Gereksinimler: 9.1, 10.1_
  
  - [ ] 4.2 Windows paket tarama ve güncelleme
    - scan_packages() - winget ve chocolatey için
    - check_package_updates() implementasyonu
    - apply_package_update() implementasyonu
    - _Gereksinimler: 1.2, 2.1, 3.1_
  
  - [ ] 4.3 Windows sürücü yönetimi
    - scan_drivers() - WMI ve Device Manager API kullanarak
    - check_driver_updates() - Windows Update API
    - backup_driver() ve restore_driver()
    - _Gereksinimler: 1.3, 4.1, 8.1, 10.1, 10.2_
  
  - [ ] 4.4 Windows yetki yönetimi
    - UAC entegrasyonu
    - request_elevation() implementasyonu
    - _Gereksinimler: 11.1, 11.8_
  
  - [ ] 4.5 Windows Adapter için property testleri
    - **Özellik 2: Paket Tarama Bütünlüğü**
    - **Özellik 3: Sürücü Tarama Bütünlüğü**
    - **Özellik 10: Platform-Spesifik Sürücü Yönetimi**
    - **Doğrular: Gereksinim 1.2, 1.3, 10.1, 10.2**

- [ ] 5. macOS Adapter'ı implement et
  - [ ] 5.1 MacOSAdapter struct ve temel metodları
    - HomebrewClient ve MasClient entegrasyonu
    - SystemUpdateClient için temel yapı
    - get_package_managers() implementasyonu
    - _Gereksinimler: 9.2, 10.3_
  
  - [ ] 5.2 macOS paket tarama ve güncelleme
    - scan_packages() - Homebrew ve mas-cli için
    - check_package_updates() implementasyonu
    - apply_package_update() implementasyonu
    - _Gereksinimler: 1.2, 2.1, 3.1_
  
  - [ ] 5.3 macOS sürücü yönetimi
    - scan_drivers() - system_profiler ve kextstat kullanarak
    - check_driver_updates() - softwareupdate komutu
    - Sürücülerin sistem güncellemeleriyle geldiğini kullanıcıya bildir
    - _Gereksinimler: 1.3, 4.1, 10.3, 10.4_
  
  - [ ] 5.4 macOS yetki yönetimi
    - macOS Authorization Services entegrasyonu
    - request_elevation() implementasyonu
    - _Gereksinimler: 11.1, 11.7_
  
  - [ ] 5.5 macOS Adapter için property testleri
    - **Özellik 2: Paket Tarama Bütünlüğü**
    - **Özellik 3: Sürücü Tarama Bütünlüğü**
    - **Özellik 10: Platform-Spesifik Sürücü Yönetimi**
    - **Doğrular: Gereksinim 1.2, 1.3, 10.3, 10.4**

- [ ] 6. Linux Adapter'ı implement et
  - [ ] 6.1 LinuxAdapter struct ve temel metodları
    - Dağıtım tespiti ve uygun paket yöneticilerini belirleme
    - FwupdClient entegrasyonu
    - get_package_managers() implementasyonu
    - _Gereksinimler: 9.3, 10.5_
  
  - [ ] 6.2 Linux paket tarama ve güncelleme
    - scan_packages() - apt, dnf, pacman, Flatpak, Snap için
    - check_package_updates() implementasyonu
    - apply_package_update() implementasyonu
    - _Gereksinimler: 1.2, 2.1, 3.1_
  
  - [ ] 6.3 Linux sürücü yönetimi
    - scan_drivers() - lsmod, modinfo, /sys/module/ kullanarak
    - check_driver_updates() - kernel ve fwupd güncellemeleri
    - dkms desteği
    - backup_driver() ve restore_driver()
    - _Gereksinimler: 1.3, 4.1, 8.1, 10.5, 10.6, 10.7_
  
  - [ ] 6.4 Linux yetki yönetimi
    - sudo ve polkit entegrasyonu
    - request_elevation() implementasyonu
    - _Gereksinimler: 11.1, 11.6_
  
  - [ ] 6.5 Linux Adapter için property testleri
    - **Özellik 2: Paket Tarama Bütünlüğü**
    - **Özellik 3: Sürücü Tarama Bütünlüğü**
    - **Özellik 10: Platform-Spesifik Sürücü Yönetimi**
    - **Doğrular: Gereksinim 1.2, 1.3, 10.5, 10.6, 10.7**

- [ ] 7. Checkpoint - Platform adaptörleri testi
  - Tüm testlerin geçtiğinden emin ol, kullanıcıya sorular varsa sor.

- [ ] 8. Core servislerini implement et
  - [ ] 8.1 UpdateScanner servisini oluştur
    - scan_system() - platform tespiti ve tarama
    - check_updates() - güncelleme kontrolü
    - ScanCache implementasyonu
    - _Gereksinimler: 1.1, 1.2, 1.3, 2.1, 2.2_
  
  - [ ] 8.2 UpdateScanner için property testleri
    - **Özellik 4: Tarama Performansı**
    - **Özellik 5: Güncelleme Bilgisi Bütünlüğü**
    - **Doğrular: Gereksinim 1.6, 2.4**
  
  - [ ] 8.3 UpdateEngine servisini oluştur
    - apply_update() - tek güncelleme uygulama
    - apply_batch_updates() - toplu güncelleme
    - Öncelik sıralaması (Critical > Important > Normal)
    - _Gereksinimler: 3.1, 3.3, 4.1, 5.1, 5.5_
  
  - [ ] 8.4 UpdateEngine için property testleri
    - **Özellik 6: Güncelleme Versiyon Değişimi**
    - **Özellik 7: Güncelleme Durum Takibi**
    - **Özellik 11: Öncelik Sıralaması**
    - **Özellik 12: Toplu Güncelleme Özeti**
    - **Doğrular: Gereksinim 3.1, 3.3, 5.3, 5.5**
  
  - [ ] 8.5 DriverBackupManager servisini oluştur
    - backup_driver() - sürücü yedekleme
    - restore_driver() - sürücü geri yükleme
    - cleanup_old_backups() - eski yedekleri temizleme (30 gün)
    - _Gereksinimler: 8.1, 8.2, 8.3, 8.4_
  
  - [ ] 8.6 DriverBackupManager için property testleri
    - **Özellik 18: Otomatik Yedekleme**
    - **Özellik 19: Yedek Konum Kaydı**
    - **Özellik 20: Sürücü Geri Yükleme (Round-trip)**
    - **Özellik 21: Yedek Saklama Süresi**
    - **Doğrular: Gereksinim 8.1, 8.2, 8.3, 8.4**

- [ ] 9. Veri katmanını implement et
  - [ ] 9.1 SQLite veritabanı şemasını oluştur
    - update_history tablosu
    - driver_backups tablosu
    - configuration tablosu
    - _Gereksinimler: 6.4_
  
  - [ ] 9.2 HistoryManager servisini oluştur
    - record_update() - güncelleme kaydı
    - get_history() - geçmiş sorgulama
    - Filtreleme ve sıralama
    - clear_history() - geçmiş temizleme
    - _Gereksinimler: 6.1, 6.2, 6.3, 6.5_
  
  - [ ] 9.3 HistoryManager için property testleri
    - **Özellik 13: Geçmiş Kayıt Bütünlüğü**
    - **Özellik 14: Geçmiş Sıralama**
    - **Özellik 15: Geçmiş Filtreleme**
    - **Özellik 16: Geçmiş Kalıcılığı (Round-trip)**
    - **Doğrular: Gereksinim 6.1, 6.2, 6.3, 6.4**
  
  - [ ] 9.4 Configuration Store'u implement et
    - Kullanıcı ayarlarını kaydetme/okuma
    - Veri şifreleme
    - _Gereksinimler: 7.4, 11.5_
  
  - [ ] 9.5 Configuration Store için property testleri
    - **Özellik 17: Ayar Kalıcılığı (Round-trip)**
    - **Özellik 32: Veri Şifreleme**
    - **Doğrular: Gereksinim 7.4, 11.5**

- [ ] 10. Checkpoint - Core servisler ve veri katmanı testi
  - Tüm testlerin geçtiğinden emin ol, kullanıcıya sorular varsa sor.

- [ ] 11. Otomatik güncelleme ve zamanlayıcı
  - [ ] 11.1 AutoUpdateScheduler servisini oluştur
    - Zamanlanmış tarama (günlük/haftalık)
    - Arka plan çalışma
    - Bildirim entegrasyonu
    - _Gereksinimler: 7.1, 7.2, 7.3, 7.5_
  
  - [ ] 11.2 AutoUpdateScheduler için unit testler
    - Zamanlama mantığı testleri
    - Arka plan kaynak kullanımı testleri
    - _Gereksinimler: 7.1, 7.5_

- [ ] 12. Güvenlik katmanını implement et
  - [ ] 12.1 Checksum doğrulama
    - calculate_checksum() fonksiyonu
    - validate_checksum() fonksiyonu
    - _Gereksinimler: 11.2_
  
  - [ ] 12.2 Checksum için property testi
    - **Özellik 29: Checksum Doğrulama**
    - **Doğrular: Gereksinim 11.2**
  
  - [ ] 12.3 Güvenilir kaynak kontrolü
    - Kaynak whitelist yönetimi
    - Kaynak doğrulama
    - _Gereksinimler: 11.3_
  
  - [ ] 12.4 Güvenlik için property testleri
    - **Özellik 30: Güvenilir Kaynak Kontrolü**
    - **Özellik 31: Hassas İşlem Loglama**
    - **Doğrular: Gereksinim 11.3, 11.4**
  
  - [ ] 12.5 Tauri 2.0 güvenlik yapılandırması
    - IPC güvenliği ayarları
    - CSP (Content Security Policy) yapılandırması
    - _Gereksinimler: 11.9_

- [ ] 13. Hata yönetimi ve loglama
  - [ ] 13.1 Hata tipleri ve kategorileri
    - Error enum tanımları
    - Kritik, Önemli, Normal kategorileri
    - _Gereksinimler: 3.4, 4.4_
  
  - [ ] 13.2 Hata işleme stratejileri
    - Network hataları
    - Güncelleme hataları
    - Sürücü hataları
    - Retry mekanizması
    - _Gereksinimler: 2.5, 3.4, 4.4, 5.4_
  
  - [ ] 13.3 Loglama sistemi
    - Log seviyeleri (Error, Warn, Info, Debug)
    - Dosya loglama
    - _Gereksinimler: 11.4_
  
  - [ ] 13.4 Hata yönetimi için unit testler
    - Hata senaryoları testleri
    - Retry mekanizması testleri
    - _Gereksinimler: 2.5, 3.4, 4.4_

- [ ] 14. Checkpoint - Backend servisleri tamamlandı
  - Tüm testlerin geçtiğinden emin ol, kullanıcıya sorular varsa sor.

- [ ] 15. Frontend UI bileşenlerini oluştur
  - [ ] 15.1 Temel UI yapısını kur
    - Ana layout bileşeni
    - Navigation/routing
    - Tema sistemi (açık/koyu)
    - _Gereksinimler: 12.1, 12.3_
  
  - [ ] 15.2 Dashboard sayfası
    - Sistem durumu gösterimi
    - Hızlı eylemler
    - _Gereksinimler: 1.4_
  
  - [ ] 15.3 Paket listesi ve güncelleme sayfası
    - Paket listesi tablosu
    - Filtreleme ve arama
    - Güncelleme seçimi
    - İlerleme gösterimi
    - _Gereksinimler: 1.4, 1.5, 2.3, 2.4, 3.2, 5.2_
  
  - [ ] 15.4 Sürücü yönetimi sayfası
    - Sürücü listesi
    - Güncelleme ve yedekleme kontrolleri
    - Yeniden başlatma uyarıları
    - _Gereksinimler: 1.4, 4.2, 4.3, 8.5_
  
  - [ ] 15.5 Geçmiş sayfası
    - Güncelleme geçmişi tablosu
    - Filtreleme (başarılı/başarısız)
    - Geçmiş temizleme
    - _Gereksinimler: 6.1, 6.3, 6.5_
  
  - [ ] 15.6 Ayarlar sayfası
    - Otomatik güncelleme ayarları
    - Tema seçimi
    - Paket yöneticisi tercihleri
    - _Gereksinimler: 7.3, 9.8, 12.3_

- [ ] 16. Tauri IPC komutlarını implement et
  - [ ] 16.1 Tauri command handler'ları
    - scan_system command
    - check_updates command
    - apply_update command
    - get_history command
    - _Gereksinimler: 1.1, 2.1, 3.1, 6.1_
  
  - [ ] 16.2 Event system
    - İlerleme event'leri
    - Hata event'leri
    - Bildirim event'leri
    - _Gereksinimler: 1.5, 3.2, 7.2_
  
  - [ ] 16.3 Tauri 2.0 IPC güvenliği
    - Command izinleri
    - IPC mesaj doğrulama
    - _Gereksinimler: 11.9_

- [ ] 17. Frontend-Backend entegrasyonu
  - [ ] 17.1 State management entegrasyonu
    - Tauri API çağrıları
    - State güncellemeleri
    - Error handling
    - _Gereksinimler: 1.1, 2.1, 3.1_
  
  - [ ] 17.2 Gerçek zamanlı güncellemeler
    - Event listener'lar
    - UI güncellemeleri
    - _Gereksinimler: 1.5, 3.2, 5.2_
  
  - [ ] 17.3 UI için integration testler
    - End-to-end akış testleri
    - _Gereksinimler: 1.1, 2.1, 3.1_

- [ ] 18. Checkpoint - UI ve entegrasyon testi
  - Tüm testlerin geçtiğinden emin ol, kullanıcıya sorular varsa sor.

- [ ] 19. Performans optimizasyonu
  - [ ] 19.1 Bellek kullanımı optimizasyonu
    - Boşta 100 MB altı hedefi
    - Bellek sızıntısı kontrolü
    - _Gereksinimler: 13.1_
  
  - [ ] 19.2 CPU kullanımı optimizasyonu
    - Tarama sırasında %20 altı hedefi
    - Asenkron işlemler
    - _Gereksinimler: 13.2, 13.3_
  
  - [ ] 19.3 Başlangıç performansı
    - 5 saniye altı başlangıç hedefi
    - Lazy loading
    - _Gereksinimler: 13.5_
  
  - [ ] 19.4 Performans testleri
    - **Özellik 37: Boşta Bellek Kullanımı**
    - **Özellik 38: Tarama CPU Kullanımı**
    - **Özellik 40: Başlangıç Performansı**
    - **Özellik 41: Çapraz Platform Performans Tutarlılığı**
    - **Doğrular: Gereksinim 13.1, 13.2, 13.5, 13.6, 13.7**

- [ ] 20. Platform-spesifik paketleme ve test
  - [ ] 20.1 Windows paketleme
    - MSI installer oluştur
    - Windows 10/11'de test et
    - _Gereksinimler: 11.1_
  
  - [ ] 20.2 macOS paketleme
    - DMG/PKG oluştur
    - macOS 12+ üzerinde test et
    - Code signing
    - _Gereksinimler: 11.1_
  
  - [ ] 20.3 Linux paketleme
    - AppImage, .deb, .rpm oluştur
    - Farklı dağıtımlarda test et
    - _Gereksinimler: 11.1_

- [ ] 21. Final checkpoint - Tüm platformlarda test
  - Tüm testlerin geçtiğinden emin ol, kullanıcıya sorular varsa sor.

## Notlar

- Tüm görevler zorunludur ve kapsamlı test kapsamı sağlar
- Her görev spesifik gereksinimlere referans verir
- Checkpoint'ler artımlı doğrulama sağlar
- Property testleri evrensel doğruluk özelliklerini doğrular
- Unit testler spesifik örnekleri ve edge case'leri doğrular
- Her property testi tasarım dokümanındaki bir özelliğe referans verir
