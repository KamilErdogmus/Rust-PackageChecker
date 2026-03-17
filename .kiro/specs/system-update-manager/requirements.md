# Gereksinimler Dokümanı

## Giriş

System Update Manager, Windows, macOS ve Linux işletim sistemlerinde çalışan, sistem paketlerini, uygulamaları ve sürücüleri otomatik olarak tespit edip güncelleyen çapraz platform masaüstü uygulamasıdır. Tauri 2.0 framework kullanılarak geliştirilecek olup (2026 standartları), Rust backend ve modern web frontend teknolojileri ile güvenli ve performanslı bir çözüm sunmayı hedefler. Her işletim sisteminin kendi paket yöneticileri ve sürücü sistemleri ile entegre çalışır.

Uygulama, her platform için özel adaptör katmanı kullanarak platform farklılıklarını soyutlar ve kullanıcıya tutarlı bir deneyim sunar. Windows'ta winget ve Chocolatey, macOS'ta Homebrew ve Mac App Store (mas-cli), Linux'ta apt, dnf, pacman, Flatpak ve Snap gibi popüler paket yöneticilerini destekler. Sürücü güncellemeleri için her platformun kendi mekanizması kullanılır: Windows'ta Windows Update ve üretici sürücüleri, macOS'ta sistem güncellemeleri (sürücüler OS güncellemeleriyle gelir), Linux'ta kernel modülleri ve firmware güncellemeleri.

## Sözlük

- **System_Update_Manager**: Ana uygulama sistemi (Tauri 2.0 tabanlı)
- **Package**: Sistemde yüklü olan yazılım paketi veya uygulama
- **Driver**: Donanım bileşenlerinin işletim sistemi ile iletişimini sağlayan yazılım
- **Update_Scanner**: Güncellemeleri tarayan bileşen
- **Update_Engine**: Güncellemeleri uygulayan bileşen
- **UI_Component**: Kullanıcı arayüzü bileşeni (Tauri 2.0 webview)
- **Backend_Service**: Rust ile yazılmış backend servisi (Tauri 2.0 core)
- **Update_Status**: Güncelleme durumu (available, downloading, installing, completed, failed)
- **Package_Manager**: İşletim sistemine özgü paket yöneticisi
  - Windows: winget (Windows Package Manager), Chocolatey
  - macOS: Homebrew, mas-cli (Mac App Store CLI)
  - Linux: apt (Debian/Ubuntu), dnf (Fedora/RHEL), pacman (Arch), Flatpak, Snap
- **Driver_System**: Platform-spesifik sürücü yönetim sistemi
  - Windows: Windows Update, üretici sürücü paketleri, Device Manager API
  - macOS: System Updates (sürücüler OS güncellemeleriyle entegre)
  - Linux: kernel modülleri, firmware-linux-nonfree, fwupd
- **Platform**: İşletim sistemi platformu (Windows 10/11, macOS 12+, Linux)
- **Platform_Adapter**: Platform-spesifik işlemleri yöneten adaptör bileşeni
- **OS_Detector**: İşletim sistemi ve versiyonunu tespit eden bileşen
- **Tauri_Runtime**: Tauri 2.0 çalışma zamanı ortamı

## Gereksinimler

### Gereksinim 1: Sistem Tarama

**Kullanıcı Hikayesi:** Bir kullanıcı olarak, sistemimde yüklü olan tüm paketleri ve sürücüleri görmek istiyorum, böylece hangi yazılımların güncel olup olmadığını kontrol edebilirim.

#### Kabul Kriterleri

1. WHEN uygulama başlatıldığında, THE System_Update_Manager SHALL çalıştığı platformu (Windows, macOS, Linux) tespit etmelidir
2. WHEN uygulama başlatıldığında, THE System_Update_Manager SHALL platform-spesifik paket yöneticilerini kullanarak sistemde yüklü tüm paketleri taramalıdır
3. WHEN uygulama başlatıldığında, THE System_Update_Manager SHALL platform-spesifik yöntemlerle sistemde yüklü tüm sürücüleri taramalıdır
4. WHEN tarama tamamlandığında, THE System_Update_Manager SHALL bulunan paket sayısını ve sürücü sayısını göstermelidir
5. WHEN tarama devam ederken, THE UI_Component SHALL kullanıcıya ilerleme durumunu göstermelidir
6. THE Update_Scanner SHALL tarama işlemini 30 saniye içinde tamamlamalıdır

### Gereksinim 2: Güncelleme Kontrolü

**Kullanıcı Hikayesi:** Bir kullanıcı olarak, yüklü paketlerim ve sürücülerim için mevcut güncellemeleri görmek istiyorum, böylece hangi bileşenlerin güncellenmesi gerektiğini bilebilirim.

#### Kabul Kriterleri

1. WHEN kullanıcı güncelleme kontrolü başlattığında, THE Update_Scanner SHALL platform-spesifik paket yöneticilerini kullanarak her paket için mevcut güncellemeleri kontrol etmelidir
2. WHEN kullanıcı güncelleme kontrolü başlattığında, THE Update_Scanner SHALL platform-spesifik yöntemlerle her sürücü için mevcut güncellemeleri kontrol etmelidir
3. WHEN güncelleme kontrolü tamamlandığında, THE System_Update_Manager SHALL mevcut güncellemeleri liste halinde göstermelidir
4. WHEN bir güncelleme bulunduğunda, THE UI_Component SHALL güncelleme bilgilerini (isim, mevcut versiyon, yeni versiyon, boyut) göstermelidir
5. IF internet bağlantısı yoksa, THEN THE System_Update_Manager SHALL kullanıcıya hata mesajı göstermelidir

### Gereksinim 3: Paket Güncelleme

**Kullanıcı Hikayesi:** Bir kullanıcı olarak, seçtiğim paketleri güncellemek istiyorum, böylece yazılımlarımı en son sürümde tutabilirim.

#### Kabul Kriterleri

1. WHEN kullanıcı bir veya birden fazla paket seçip güncelleme başlattığında, THE Update_Engine SHALL seçilen paketleri güncellemelidir
2. WHILE güncelleme devam ederken, THE UI_Component SHALL her paket için ilerleme durumunu göstermelidir
3. WHEN bir paket güncellemesi tamamlandığında, THE System_Update_Manager SHALL güncelleme durumunu "completed" olarak işaretlemelidir
4. IF bir paket güncellemesi başarısız olursa, THEN THE System_Update_Manager SHALL hata mesajını kaydetmeli ve kullanıcıya göstermelidir
5. THE Update_Engine SHALL güncelleme işlemi sırasında sistem kararlılığını korumalıdır

### Gereksinim 4: Sürücü Güncelleme

**Kullanıcı Hikayesi:** Bir kullanıcı olarak, sistemimde yüklü sürücüleri güncellemek istiyorum, böylece donanımlarımın en iyi performansta çalışmasını sağlayabilirim.

#### Kabul Kriterleri

1. WHEN kullanıcı bir veya birden fazla sürücü seçip güncelleme başlattığında, THE Update_Engine SHALL platform-spesifik yöntemlerle seçilen sürücüleri güncellemelidir
2. WHEN sürücü güncellemesi başlatıldığında, THE System_Update_Manager SHALL kullanıcıya sistem yeniden başlatma gerekebileceği uyarısını göstermelidir
3. WHILE sürücü güncellemesi devam ederken, THE UI_Component SHALL güncelleme ilerlemesini göstermelidir
4. WHEN sürücü güncellemesi tamamlandığında, THE System_Update_Manager SHALL kullanıcıya yeniden başlatma seçeneği sunmalıdır
5. IF sürücü güncellemesi başarısız olursa, THEN THE System_Update_Manager SHALL platform-spesifik geri yükleme mekanizmasını kullanarak önceki sürücü versiyonunu geri yükleme seçeneği sunmalıdır

### Gereksinim 5: Toplu Güncelleme

**Kullanıcı Hikayesi:** Bir kullanıcı olarak, tüm mevcut güncellemeleri tek seferde yüklemek istiyorum, böylece her güncellemeyi tek tek seçmek zorunda kalmam.

#### Kabul Kriterleri

1. WHEN kullanıcı "Tümünü Güncelle" butonuna tıkladığında, THE Update_Engine SHALL tüm mevcut güncellemeleri sıraya koymalıdır
2. WHILE toplu güncelleme devam ederken, THE UI_Component SHALL genel ilerleme durumunu ve güncel güncellenen öğeyi göstermelidir
3. WHEN toplu güncelleme tamamlandığında, THE System_Update_Manager SHALL özet rapor göstermelidir (başarılı, başarısız, atlanan)
4. IF toplu güncelleme sırasında kritik bir hata oluşursa, THEN THE System_Update_Manager SHALL güncelleme işlemini duraklayıp kullanıcıya seçenek sunmalıdır
5. THE Update_Engine SHALL güncellemeleri öncelik sırasına göre (kritik, önemli, normal) uygulamalıdır

### Gereksinim 6: Güncelleme Geçmişi

**Kullanıcı Hikayesi:** Bir kullanıcı olarak, geçmişte yapılan güncellemeleri görmek istiyorum, böylece hangi paketlerin ne zaman güncellendiğini takip edebilirim.

#### Kabul Kriterleri

1. WHEN kullanıcı güncelleme geçmişi sayfasını açtığında, THE System_Update_Manager SHALL tüm geçmiş güncellemeleri tarih sırasına göre göstermelidir
2. WHEN bir güncelleme tamamlandığında, THE System_Update_Manager SHALL güncelleme kaydını (tarih, paket adı, eski versiyon, yeni versiyon, durum) kaydetmelidir
3. WHERE kullanıcı filtreleme seçeneğini kullanırsa, THE UI_Component SHALL güncellemeleri duruma göre (başarılı, başarısız) filtrelemelidir
4. THE System_Update_Manager SHALL güncelleme geçmişini yerel veritabanında saklamalıdır
5. WHEN kullanıcı geçmiş kaydını temizlemek isterse, THE System_Update_Manager SHALL onay isteyip ardından kayıtları silmelidir

### Gereksinim 7: Otomatik Güncelleme Kontrolü

**Kullanıcı Hikayesi:** Bir kullanıcı olarak, uygulamanın otomatik olarak güncellemeleri kontrol etmesini istiyorum, böylece manuel kontrol yapmak zorunda kalmam.

#### Kabul Kriterleri

1. WHERE otomatik kontrol etkinleştirilmişse, THE Update_Scanner SHALL belirlenen aralıklarla (günlük, haftalık) güncelleme kontrolü yapmalıdır
2. WHEN otomatik kontrol yeni güncelleme bulduğunda, THE System_Update_Manager SHALL kullanıcıya bildirim göstermelidir
3. WHERE kullanıcı ayarlardan otomatik kontrolü devre dışı bırakırsa, THE Update_Scanner SHALL otomatik kontrolleri durdurmalıdır
4. THE System_Update_Manager SHALL otomatik kontrol ayarlarını kullanıcı tercihlerine göre saklamalıdır
5. WHEN uygulama arka planda çalışırken, THE Update_Scanner SHALL sistem kaynaklarını minimal düzeyde kullanmalıdır

### Gereksinim 8: Sürücü Yedekleme

**Kullanıcı Hikayesi:** Bir kullanıcı olarak, sürücü güncellemeden önce mevcut sürücüleri yedeklemek istiyorum, böylece sorun çıkarsa eski sürücüye geri dönebilirim.

#### Kabul Kriterleri

1. WHEN sürücü güncellemesi başlatıldığında, THE System_Update_Manager SHALL platform-spesifik yöntemlerle mevcut sürücüyü otomatik olarak yedeklemelidir
2. WHEN yedekleme tamamlandığında, THE System_Update_Manager SHALL yedek dosyasının konumunu kaydetmelidir
3. WHERE kullanıcı sürücü geri yükleme seçeneğini kullanırsa, THE System_Update_Manager SHALL platform-spesifik geri yükleme mekanizmasını kullanarak yedeklenmiş sürücüyü geri yüklemelidir
4. THE System_Update_Manager SHALL sürücü yedeklerini en az 30 gün boyunca saklamalıdır
5. WHEN disk alanı yetersiz olduğunda, THE System_Update_Manager SHALL kullanıcıya uyarı göstermeli ve eski yedekleri temizleme seçeneği sunmalıdır

### Gereksinim 9: Paket Yöneticisi Entegrasyonu

**Kullanıcı Hikayesi:** Bir sistem yöneticisi olarak, farklı platformlardaki paket yöneticilerini desteklemek istiyorum, böylece farklı kaynaklardan yüklenen paketleri yönetebilirim.

#### Kabul Kriterleri

1. WHERE platform Windows ise, THE Backend_Service SHALL Windows Package Manager (winget) ve Chocolatey ile entegre olmalıdır
2. WHERE platform macOS ise, THE Backend_Service SHALL Homebrew ve mas-cli (Mac App Store Command Line Interface) ile entegre olmalıdır
3. WHERE platform Linux ise, THE Backend_Service SHALL apt (Debian/Ubuntu), dnf (Fedora/RHEL), pacman (Arch), Flatpak ve Snap ile entegre olmalıdır
4. WHEN paket güncellemesi yapılırken, THE Update_Engine SHALL ilgili paket yöneticisini kullanmalıdır
5. THE System_Update_Manager SHALL her paket için hangi paket yöneticisi tarafından yönetildiğini göstermelidir
6. IF bir paket yöneticisi sistemde yoksa, THEN THE System_Update_Manager SHALL kullanıcıya kurulum önerisi sunmalıdır
7. THE Platform_Adapter SHALL her platform için uygun paket yöneticisi komutlarını çalıştırmalıdır
8. WHEN birden fazla paket yöneticisi aynı paketi yönetiyorsa, THE System_Update_Manager SHALL kullanıcıya tercih seçeneği sunmalıdır

### Gereksinim 10: Platform-Spesifik Sürücü Yönetimi

**Kullanıcı Hikayesi:** Bir kullanıcı olarak, her platformun kendi sürücü yönetim sistemini kullanarak sürücü güncellemelerini almak istiyorum, böylece donanımlarım en uygun şekilde desteklensin.

#### Kabul Kriterleri

1. WHERE platform Windows ise, THE Driver_System SHALL Windows Update API ve üretici sürücü paketlerini kullanmalıdır
2. WHERE platform Windows ise, THE Driver_System SHALL Device Manager API üzerinden sürücü bilgilerini almalıdır
3. WHERE platform macOS ise, THE Driver_System SHALL sistem güncellemeleri üzerinden sürücü güncellemelerini kontrol etmelidir (macOS'ta sürücüler OS güncellemeleriyle entegre)
4. WHERE platform macOS ise, THE System_Update_Manager SHALL kullanıcıya sürücülerin sistem güncellemeleriyle geldiğini bildirmelidir
5. WHERE platform Linux ise, THE Driver_System SHALL kernel modüllerini ve firmware güncellemelerini yönetmelidir
6. WHERE platform Linux ise, THE Driver_System SHALL fwupd daemon ile entegre olarak firmware güncellemelerini kontrol etmelidir
7. WHERE platform Linux ise, THE Driver_System SHALL dkms (Dynamic Kernel Module Support) ile uyumlu çalışmalıdır
8. WHEN sürücü güncellemesi yapılırken, THE Platform_Adapter SHALL platform-spesifik güvenlik mekanizmalarını kullanmalıdır
9. THE System_Update_Manager SHALL her platform için uygun sürücü bilgilerini (isim, versiyon, üretici, donanım ID) göstermelidir

### Gereksinim 11: Çapraz Platform Desteği

**Kullanıcı Hikayesi:** Bir kullanıcı olarak, uygulamanın Windows, macOS ve Linux üzerinde tutarlı şekilde çalışmasını istiyorum, böylece hangi işletim sistemini kullanırsam kullanayım aynı deneyimi yaşayabilirim.

#### Kabul Kriterleri

1. THE System_Update_Manager SHALL Windows 10/11, macOS 12+ ve modern Linux dağıtımlarında çalışmalıdır
2. WHEN uygulama başlatıldığında, THE OS_Detector SHALL çalışılan platformu ve versiyonunu otomatik olarak tespit etmelidir
3. THE Platform_Adapter SHALL her platform için özel sistem çağrılarını ve komutlarını yönetmelidir
4. THE UI_Component SHALL Tauri 2.0 webview kullanarak tüm platformlarda tutarlı görünüm ve davranış sergilemelidir
5. WHERE platform-spesifik özellikler varsa, THE System_Update_Manager SHALL bu özellikleri sadece ilgili platformda göstermelidir
6. THE Backend_Service SHALL platform farklılıklarını soyutlayan bir mimari kullanmalıdır
7. WHEN platform değiştiğinde, THE System_Update_Manager SHALL yapılandırma ve veri formatlarını koruyarak çalışmalıdır
8. THE Tauri_Runtime SHALL her platformda native performans sağlamalıdır

### Gereksinim 12: Güvenlik ve İzinler

**Kullanıcı Hikayesi:** Bir kullanıcı olarak, uygulamanın güvenli bir şekilde çalışmasını istiyorum, böylece sistem güvenliğim tehlikeye girmesin.

#### Kabul Kriterleri

1. WHEN güncelleme işlemi yönetici yetkisi gerektirdiğinde, THE System_Update_Manager SHALL platform-spesifik yetki yükseltme mekanizmasını kullanarak kullanıcıdan izin istemelidir
2. THE Backend_Service SHALL tüm indirilen dosyaların bütünlüğünü checksum ile doğrulamalıdır
3. THE System_Update_Manager SHALL sadece güvenilir kaynaklardan güncelleme indirmelidir
4. WHEN hassas işlem yapılırken, THE System_Update_Manager SHALL işlemi güvenli bir şekilde loglamalıdır
5. THE Backend_Service SHALL kullanıcı verilerini şifreleyerek saklamalıdır
6. WHERE platform Linux ise, THE System_Update_Manager SHALL sudo veya polkit kullanarak yetki yönetimi yapmalıdır
7. WHERE platform macOS ise, THE System_Update_Manager SHALL macOS authorization services kullanmalıdır
8. WHERE platform Windows ise, THE System_Update_Manager SHALL UAC (User Account Control) ile entegre olmalıdır
9. THE Tauri_Runtime SHALL Tauri 2.0 güvenlik özelliklerini (IPC güvenliği, CSP) kullanmalıdır

### Gereksinim 13: Kullanıcı Arayüzü

**Kullanıcı Hikayesi:** Bir kullanıcı olarak, modern ve kullanımı kolay bir arayüz istiyorum, böylece uygulamayı rahatça kullanabilirim.

#### Kabul Kriterleri

1. THE UI_Component SHALL Tauri 2.0 webview kullanarak responsive tasarıma sahip olmalıdır
2. WHEN kullanıcı bir işlem başlattığında, THE UI_Component SHALL anında görsel geri bildirim vermelidir
3. THE UI_Component SHALL açık ve koyu tema seçenekleri sunmalıdır
4. WHEN hata oluştuğunda, THE UI_Component SHALL kullanıcı dostu hata mesajları göstermelidir
5. THE UI_Component SHALL WCAG 2.1 erişilebilirlik standartlarına uygun olmalıdır
6. THE UI_Component SHALL tüm platformlarda (Windows, macOS, Linux) native görünüm ve his sunmalıdır
7. THE Tauri_Runtime SHALL her platformda native pencere kontrollerini kullanmalıdır

### Gereksinim 14: Performans ve Kaynak Yönetimi

**Kullanıcı Hikayesi:** Bir kullanıcı olarak, uygulamanın sistem kaynaklarını verimli kullanmasını istiyorum, böylece bilgisayarımın performansı etkilenmesin.

#### Kabul Kriterleri

1. THE Backend_Service SHALL boşta iken maksimum 100 MB RAM kullanmalıdır
2. WHILE tarama işlemi devam ederken, THE Update_Scanner SHALL CPU kullanımını %20'nin altında tutmalıdır
3. THE System_Update_Manager SHALL indirme işlemlerini arka planda asenkron olarak yapmalıdır
4. WHEN birden fazla güncelleme indirilirken, THE Update_Engine SHALL bant genişliğini adil şekilde paylaştırmalıdır
5. THE System_Update_Manager SHALL 5 saniye içinde başlamalıdır
6. THE Platform_Adapter SHALL tüm platformlarda benzer performans özellikleri göstermelidir
7. THE Tauri_Runtime SHALL native performans sağlamak için Rust optimizasyonlarını kullanmalıdır
