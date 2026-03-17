# Tasarım Dokümanı: System Update Manager

## Genel Bakış

System Update Manager, Windows, macOS ve Linux işletim sistemlerinde çalışan çapraz platform bir masaüstü uygulamasıdır. Tauri 2.0 framework (2026 standartları) kullanılarak geliştirilecek olup, Rust backend ve modern web frontend (React/TypeScript) teknolojileri ile güvenli ve performanslı bir çözüm sunar.

Uygulama, her platformun kendi paket yöneticileri ve sürücü sistemleri ile entegre çalışarak:
- Sistem paketlerini ve uygulamaları tarar
- Mevcut güncellemeleri kontrol eder
- Güncellemeleri güvenli bir şekilde uygular
- Platform-spesifik sürücü güncellemelerini yönetir
- Güncelleme geçmişini takip eder

### Platform-Spesifik Entegrasyonlar

**Windows (10/11)**:
- Paket Yöneticileri: winget (Windows Package Manager), Chocolatey
- Sürücü Sistemi: Windows Update API, Device Manager API, üretici sürücü paketleri
- Yetki Yönetimi: UAC (User Account Control)

**macOS (12+)**:
- Paket Yöneticileri: Homebrew, mas-cli (Mac App Store Command Line Interface)
- Sürücü Sistemi: System Updates (sürücüler OS güncellemeleriyle entegre)
- Yetki Yönetimi: macOS Authorization Services

**Linux (Modern Dağıtımlar)**:
- Paket Yöneticileri: apt (Debian/Ubuntu), dnf (Fedora/RHEL), pacman (Arch), Flatpak, Snap
- Sürücü Sistemi: kernel modülleri, fwupd (firmware updates), dkms (Dynamic Kernel Module Support)
- Yetki Yönetimi: sudo, polkit

### Temel Özellikler

- **Çapraz Platform Desteği**: Windows 10/11, macOS 12+ ve modern Linux dağıtımları
- **Tauri 2.0 Mimarisi**: Native performans, güvenli IPC, minimal kaynak kullanımı
- **Çoklu Paket Yöneticisi Entegrasyonu**: Her platformun yerel paket yöneticileri ile entegre
- **Platform-Spesifik Sürücü Yönetimi**: Her platformun kendi sürücü sistemini kullanır
- **Otomatik Platform Tespiti**: OS_Detector ile otomatik platform ve versiyon tespiti
- **Otomatik Güncelleme Kontrolü**: Zamanlanmış güncelleme taramaları
- **Güvenli Güncelleme**: Checksum doğrulama, yedekleme ve Tauri 2.0 güvenlik özellikleri
- **Modern UI**: Responsive, erişilebilir, native görünüm ve tema desteği

## Mimari

### Genel Mimari

Uygulama, katmanlı bir mimari kullanır:

```
┌─────────────────────────────────────────┐
│    Frontend (React/TS + Tauri 2.0)     │
│  - UI Components                        │
│  - State Management (Zustand/Redux)    │
│  - Tauri 2.0 API Calls                 │
│  - Native Window Controls              │
└─────────────────────────────────────────┘
                  ↕
┌─────────────────────────────────────────┐
│      Tauri 2.0 Bridge & IPC             │
│  - Secure IPC Communication             │
│  - Command Handlers                     │
│  - Event System                         │
│  - CSP (Content Security Policy)        │
└─────────────────────────────────────────┘
                  ↕
┌─────────────────────────────────────────┐
│         Backend (Rust)                  │
│  ┌───────────────────────────────────┐  │
│  │   OS Detection Layer              │  │
│  │   - OS_Detector                   │  │
│  │   - Version Detection             │  │
│  └───────────────────────────────────┘  │
│                  ↕                      │
│  ┌───────────────────────────────────┐  │
│  │   Platform Abstraction Layer      │  │
│  │   - Platform Adapter Trait        │  │
│  └───────────────────────────────────┘  │
│           ↕          ↕          ↕        │
│  ┌─────────┐  ┌─────────┐  ┌─────────┐ │
│  │ Windows │  │  macOS  │  │  Linux  │ │
│  │ Adapter │  │ Adapter │  │ Adapter │ │
│  │ (winget,│  │(Homebrew│  │(apt,dnf,│ │
│  │  Choco, │  │ mas-cli,│  │ pacman, │ │
│  │ WinUpdt)│  │ SysUpdt)│  │fwupd)   │ │
│  └─────────┘  └─────────┘  └─────────┘ │
│                                         │
│  ┌───────────────────────────────────┐  │
│  │   Core Services                   │  │
│  │   - Update Scanner                │  │
│  │   - Update Engine                 │  │
│  │   - Driver Manager                │  │
│  │   - History Manager               │  │
│  │   - Auto Update Scheduler         │  │
│  └───────────────────────────────────┘  │
│                                         │
│  ┌───────────────────────────────────┐  │
│  │   Data Layer                      │  │
│  │   - SQLite Database               │  │
│  │   - Configuration Store           │  │
│  │   - Encrypted User Data           │  │
│  └───────────────────────────────────┘  │
└─────────────────────────────────────────┘
```

### Platform Abstraction Pattern

Çapraz platform desteği için Adapter Pattern kullanılır. Her platform için ayrı bir adapter implementasyonu bulunur.

#### OS Detection

Platform tespiti için OS_Detector bileşeni kullanılır:

```rust
pub struct OS_Detector;

impl OS_Detector {
    pub fn detect() -> Platform {
        #[cfg(target_os = "windows")]
        {
            let version = Self::get_windows_version();
            Platform::Windows(version)
        }
        
        #[cfg(target_os = "macos")]
        {
            let version = Self::get_macos_version();
            Platform::MacOS(version)
        }
        
        #[cfg(target_os = "linux")]
        {
            let distro = Self::detect_linux_distro();
            Platform::Linux(distro)
        }
    }
    
    fn get_windows_version() -> WindowsVersion {
        // Windows 10 veya 11 tespiti
        // Registry veya WMI kullanarak
    }
    
    fn get_macos_version() -> MacOSVersion {
        // sw_vers komutu ile versiyon tespiti
    }
    
    fn detect_linux_distro() -> LinuxDistro {
        // /etc/os-release dosyasını oku
        // Dağıtım ve versiyonu tespit et
    }
}
```

#### Platform Adapter Trait

Platform-spesifik işlemleri soyutlayan ana trait:

```rust
pub trait PlatformAdapter: Send + Sync {
    // Platform tespiti
    fn detect_platform(&self) -> Platform;
    
    // Paket yöneticisi işlemleri
    fn get_package_managers(&self) -> Vec<PackageManager>;
    fn scan_packages(&self, manager: &PackageManager) -> Result<Vec<Package>>;
    fn check_package_updates(&self, packages: &[Package]) -> Result<Vec<Update>>;
    fn apply_package_update(&self, update: &Update) -> Result<UpdateResult>;
    
    // Sürücü işlemleri
    fn scan_drivers(&self) -> Result<Vec<Driver>>;
    fn check_driver_updates(&self, drivers: &[Driver]) -> Result<Vec<DriverUpdate>>;
    fn apply_driver_update(&self, update: &DriverUpdate) -> Result<UpdateResult>;
    fn backup_driver(&self, driver: &Driver) -> Result<PathBuf>;
    fn restore_driver(&self, backup_path: &PathBuf) -> Result<()>;
    
    // Yetki yönetimi
    fn requires_elevation(&self, operation: &Operation) -> bool;
    fn request_elevation(&self) -> Result<()>;
}
```

## Bileşenler ve Arayüzler

### 1. Platform Adapter Layer

Her platform için özel implementasyonlar:
- **WindowsAdapter**: winget, Chocolatey, Windows Update API, Device Manager API
- **MacOSAdapter**: Homebrew, mas-cli, System Updates
- **LinuxAdapter**: apt, dnf, pacman, Flatpak, Snap, fwupd

#### Windows Adapter

Windows platformu için özel implementasyon:

```rust
pub struct WindowsAdapter {
    winget_client: WingetClient,
    chocolatey_client: Option<ChocolateyClient>,
    windows_update_client: WindowsUpdateClient,
}

impl PlatformAdapter for WindowsAdapter {
    fn get_package_managers(&self) -> Vec<PackageManager> {
        let mut managers = vec![PackageManager::Winget];
        if self.chocolatey_client.is_some() {
            managers.push(PackageManager::Chocolatey);
        }
        managers
    }
    
    fn scan_packages(&self, manager: &PackageManager) -> Result<Vec<Package>> {
        match manager {
            PackageManager::Winget => self.winget_client.list_packages(),
            PackageManager::Chocolatey => {
                self.chocolatey_client
                    .as_ref()
                    .ok_or(Error::PackageManagerNotFound)?
                    .list_packages()
            }
            _ => Err(Error::UnsupportedPackageManager),
        }
    }
    
    fn scan_drivers(&self) -> Result<Vec<Driver>> {
        // Windows Device Manager API kullanarak sürücüleri tara
        // WMI (Windows Management Instrumentation) ile sürücü bilgilerini al
        // SetupAPI kullanarak donanım bilgilerini topla
        self.windows_update_client.enumerate_drivers()
    }
    
    fn check_driver_updates(&self, drivers: &[Driver]) -> Result<Vec<DriverUpdate>> {
        // Windows Update API ile sürücü güncellemelerini kontrol et
        // Üretici web sitelerinden güncelleme kontrolü (opsiyonel)
        self.windows_update_client.check_driver_updates(drivers)
    }
    
    fn backup_driver(&self, driver: &Driver) -> Result<PathBuf> {
        // Driver Store'dan sürücü dosyalarını yedekle
        // Registry ayarlarını kaydet
    }
    
    fn request_elevation(&self) -> Result<()> {
        // UAC (User Account Control) ile yetki yükseltme
        // ShellExecute ile "runas" verb kullan
    }
}
```

#### macOS Adapter

macOS platformu için özel implementasyon:

```rust
pub struct MacOSAdapter {
    homebrew_client: Option<HomebrewClient>,
    mas_client: Option<MasClient>, // Mac App Store CLI
    system_update_client: SystemUpdateClient,
}

impl PlatformAdapter for MacOSAdapter {
    fn get_package_managers(&self) -> Vec<PackageManager> {
        let mut managers = Vec::new();
        if self.homebrew_client.is_some() {
            managers.push(PackageManager::Homebrew);
        }
        if self.mas_client.is_some() {
            managers.push(PackageManager::MacAppStore);
        }
        managers
    }
    
    fn scan_packages(&self, manager: &PackageManager) -> Result<Vec<Package>> {
        match manager {
            PackageManager::Homebrew => {
                self.homebrew_client
                    .as_ref()
                    .ok_or(Error::PackageManagerNotFound)?
                    .list_packages()
            }
            PackageManager::MacAppStore => {
                self.mas_client
                    .as_ref()
                    .ok_or(Error::PackageManagerNotFound)?
                    .list_packages()
            }
            _ => Err(Error::UnsupportedPackageManager),
        }
    }
    
    fn scan_drivers(&self) -> Result<Vec<Driver>> {
        // macOS'ta sürücüler genellikle sistem güncellemeleri ile gelir
        // system_profiler SPSoftwareDataType kullanarak bilgi al
        // kextstat komutu ile kernel extension'ları listele
        // IOKit framework ile donanım bilgilerini al
        self.system_update_client.enumerate_drivers()
    }
    
    fn check_driver_updates(&self, drivers: &[Driver]) -> Result<Vec<DriverUpdate>> {
        // macOS'ta sürücüler sistem güncellemeleriyle gelir
        // softwareupdate komutu ile kontrol et
        self.system_update_client.check_system_updates()
    }
    
    fn backup_driver(&self, driver: &Driver) -> Result<PathBuf> {
        // Kernel extension'ı yedekle
        // /Library/Extensions/ veya /System/Library/Extensions/ dizininden
    }
    
    fn request_elevation(&self) -> Result<()> {
        // macOS Authorization Services kullan
        // AuthorizationExecuteWithPrivileges (deprecated) yerine
        // SMJobBless veya osascript ile sudo kullan
    }
}
```

#### Linux Adapter

Linux platformu için özel implementasyon:

```rust
pub struct LinuxAdapter {
    distro: LinuxDistro,
    package_managers: Vec<PackageManager>,
    fwupd_client: Option<FwupdClient>, // Firmware updates
}

impl PlatformAdapter for LinuxAdapter {
    fn get_package_managers(&self) -> Vec<PackageManager> {
        // Dağıtıma göre mevcut paket yöneticilerini tespit et
        match self.distro {
            LinuxDistro::Debian | LinuxDistro::Ubuntu => {
                vec![PackageManager::Apt, PackageManager::Snap, PackageManager::Flatpak]
            }
            LinuxDistro::Fedora | LinuxDistro::RHEL => {
                vec![PackageManager::Dnf, PackageManager::Flatpak]
            }
            LinuxDistro::Arch => {
                vec![PackageManager::Pacman, PackageManager::Flatpak]
            }
            LinuxDistro::Other(_) => {
                // Mevcut paket yöneticilerini tespit et
                self.detect_available_package_managers()
            }
        }
    }
    
    fn scan_drivers(&self) -> Result<Vec<Driver>> {
        // lsmod komutu ile yüklü kernel modüllerini listele
        // modinfo ile modül detaylarını al
        // /sys/module/ dizinini tara
        // lspci ve lsusb ile donanım bilgilerini al
        self.enumerate_kernel_modules()
    }
    
    fn check_driver_updates(&self, drivers: &[Driver]) -> Result<Vec<DriverUpdate>> {
        let mut updates = Vec::new();
        
        // Kernel güncellemelerini kontrol et
        updates.extend(self.check_kernel_updates()?);
        
        // fwupd ile firmware güncellemelerini kontrol et
        if let Some(fwupd) = &self.fwupd_client {
            updates.extend(fwupd.check_firmware_updates()?);
        }
        
        Ok(updates)
    }
    
    fn backup_driver(&self, driver: &Driver) -> Result<PathBuf> {
        // Kernel modülünü yedekle
        // /lib/modules/$(uname -r)/ dizininden
        // dkms kullanarak modül bilgilerini kaydet
    }
    
    fn request_elevation(&self) -> Result<()> {
        // sudo veya polkit kullan
        // pkexec komutu ile GUI uygulamalar için
        // sudo ile CLI komutları için
    }
}
```

### 2. Update Scanner

Güncellemeleri tarayan ve tespit eden bileşen:

```rust
pub struct UpdateScanner {
    platform_adapter: Arc<dyn PlatformAdapter>,
    cache: Arc<RwLock<ScanCache>>,
}

impl UpdateScanner {
    pub async fn scan_system(&self) -> Result<ScanResult> {
        let platform = self.platform_adapter.detect_platform();
        let package_managers = self.platform_adapter.get_package_managers();
        
        let mut all_packages = Vec::new();
        for manager in package_managers {
            let packages = self.platform_adapter.scan_packages(&manager).await?;
            all_packages.extend(packages);
        }
        
        let drivers = self.platform_adapter.scan_drivers().await?;
        
        Ok(ScanResult {
            platform,
            packages: all_packages,
            drivers,
            scan_time: Utc::now(),
        })
    }
    
    pub async fn check_updates(&self, scan_result: &ScanResult) -> Result<Vec<Update>> {
        let package_updates = self.platform_adapter
            .check_package_updates(&scan_result.packages)
            .await?;
            
        let driver_updates = self.platform_adapter
            .check_driver_updates(&scan_result.drivers)
            .await?;
        
        let mut all_updates = Vec::new();
        all_updates.extend(package_updates.into_iter().map(Update::Package));
        all_updates.extend(driver_updates.into_iter().map(Update::Driver));
        
        Ok(all_updates)
    }
}
```

### 3. Update Engine

Güncellemeleri uygulayan bileşen:

```rust
pub struct UpdateEngine {
    platform_adapter: Arc<dyn PlatformAdapter>,
    history_manager: Arc<HistoryManager>,
    driver_backup_manager: Arc<DriverBackupManager>,
}

impl UpdateEngine {
    pub async fn apply_update(&self, update: &Update) -> Result<UpdateResult> {
        // Yetki kontrolü
        if self.platform_adapter.requires_elevation(&update.operation) {
            self.platform_adapter.request_elevation()?;
        }
        
        match update {
            Update::Package(pkg_update) => {
                self.apply_package_update(pkg_update).await
            }
            Update::Driver(drv_update) => {
                self.apply_driver_update(drv_update).await
            }
        }
    }
    
    async fn apply_driver_update(&self, update: &DriverUpdate) -> Result<UpdateResult> {
        // Sürücüyü yedekle
        let backup_path = self.driver_backup_manager
            .backup_driver(&update.driver)
            .await?;
        
        // Güncellemeyi uygula
        let result = self.platform_adapter
            .apply_driver_update(update)
            .await;
        
        match result {
            Ok(update_result) => {
                // Başarılı güncellemeyi kaydet
                self.history_manager.record_update(update, &update_result).await?;
                Ok(update_result)
            }
            Err(e) => {
                // Hata durumunda geri yükle
                self.platform_adapter.restore_driver(&backup_path).await?;
                Err(e)
            }
        }
    }
    
    pub async fn apply_batch_updates(&self, updates: Vec<Update>) -> Result<BatchUpdateResult> {
        // Güncellemeleri öncelik sırasına göre sırala
        let sorted_updates = self.sort_by_priority(updates);
        
        let mut results = Vec::new();
        for update in sorted_updates {
            let result = self.apply_update(&update).await;
            results.push((update, result));
            
            // Kritik hata durumunda dur
            if let Err(ref e) = results.last().unwrap().1 {
                if e.is_critical() {
                    break;
                }
            }
        }
        
        Ok(BatchUpdateResult { results })
    }
}
```

### 4. Driver Backup Manager

Sürücü yedekleme ve geri yükleme işlemlerini yöneten bileşen:

```rust
pub struct DriverBackupManager {
    backup_dir: PathBuf,
    platform_adapter: Arc<dyn PlatformAdapter>,
    retention_days: u32,
}

impl DriverBackupManager {
    pub async fn backup_driver(&self, driver: &Driver) -> Result<PathBuf> {
        let backup_path = self.generate_backup_path(driver);
        self.platform_adapter.backup_driver(driver).await?;
        
        // Metadata kaydet
        self.save_backup_metadata(&backup_path, driver).await?;
        
        Ok(backup_path)
    }
    
    pub async fn restore_driver(&self, backup_path: &PathBuf) -> Result<()> {
        self.platform_adapter.restore_driver(backup_path).await
    }
    
    pub async fn cleanup_old_backups(&self) -> Result<()> {
        let cutoff_date = Utc::now() - Duration::days(self.retention_days as i64);
        
        // Eski yedekleri temizle
        for entry in fs::read_dir(&self.backup_dir)? {
            let entry = entry?;
            let metadata = entry.metadata()?;
            
            if let Ok(modified) = metadata.modified() {
                let modified_date: DateTime<Utc> = modified.into();
                if modified_date < cutoff_date {
                    fs::remove_file(entry.path())?;
                }
            }
        }
        
        Ok(())
    }
}
```

### 5. History Manager

Güncelleme geçmişini yöneten bileşen:

```rust
pub struct HistoryManager {
    db: Arc<Database>,
}

impl HistoryManager {
    pub async fn record_update(&self, update: &Update, result: &UpdateResult) -> Result<()> {
        let record = UpdateRecord {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            update_type: update.update_type(),
            package_name: update.name(),
            old_version: update.old_version(),
            new_version: update.new_version(),
            status: result.status.clone(),
            error_message: result.error.as_ref().map(|e| e.to_string()),
        };
        
        self.db.insert_update_record(&record).await
    }
    
    pub async fn get_history(&self, filter: HistoryFilter) -> Result<Vec<UpdateRecord>> {
        self.db.query_update_history(&filter).await
    }
    
    pub async fn clear_history(&self) -> Result<()> {
        self.db.clear_update_history().await
    }
}
```

### 6. Auto Update Scheduler

Otomatik güncelleme kontrolü için zamanlayıcı:

```rust
pub struct AutoUpdateScheduler {
    scanner: Arc<UpdateScanner>,
    config: Arc<RwLock<SchedulerConfig>>,
    notification_service: Arc<NotificationService>,
}

impl AutoUpdateScheduler {
    pub async fn start(&self) {
        loop {
            let config = self.config.read().await;
            
            if !config.enabled {
                tokio::time::sleep(Duration::from_secs(60)).await;
                continue;
            }
            
            let interval = match config.interval {
                ScheduleInterval::Daily => Duration::from_secs(86400),
                ScheduleInterval::Weekly => Duration::from_secs(604800),
            };
            
            drop(config);
            
            // Güncelleme kontrolü yap
            if let Ok(scan_result) = self.scanner.scan_system().await {
                if let Ok(updates) = self.scanner.check_updates(&scan_result).await {
                    if !updates.is_empty() {
                        self.notification_service.notify_updates_available(updates.len()).await;
                    }
                }
            }
            
            tokio::time::sleep(interval).await;
        }
    }
}
```

## Veri Modelleri

### Platform

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Platform {
    Windows(WindowsVersion),
    MacOS(MacOSVersion),
    Linux(LinuxDistro),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WindowsVersion {
    Windows10,
    Windows11,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MacOSVersion {
    Monterey,    // 12
    Ventura,     // 13
    Sonoma,      // 14
    Sequoia,     // 15
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LinuxDistro {
    Debian,
    Ubuntu,
    Fedora,
    RHEL,
    Arch,
    Other(String),
}
```

### Package Manager

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PackageManager {
    // Windows
    Winget,
    Chocolatey,
    
    // macOS
    Homebrew,
    MacAppStore,  // mas-cli
    
    // Linux
    Apt,
    Dnf,
    Pacman,
    Flatpak,
    Snap,
}
```

### Package

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Package {
    pub id: String,
    pub name: String,
    pub version: String,
    pub manager: PackageManager,
    pub description: Option<String>,
    pub installed_date: Option<DateTime<Utc>>,
}
```

### Driver

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Driver {
    pub id: String,
    pub name: String,
    pub version: String,
    pub device_type: DeviceType,
    pub manufacturer: String,
    pub driver_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeviceType {
    Graphics,
    Network,
    Audio,
    Storage,
    Input,
    Other(String),
}
```

### Update

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Update {
    Package(PackageUpdate),
    Driver(DriverUpdate),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageUpdate {
    pub package: Package,
    pub new_version: String,
    pub size_bytes: Option<u64>,
    pub priority: UpdatePriority,
    pub changelog: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriverUpdate {
    pub driver: Driver,
    pub new_version: String,
    pub size_bytes: Option<u64>,
    pub requires_reboot: bool,
    pub release_notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum UpdatePriority {
    Critical,
    Important,
    Normal,
}
```

### Update Result

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateResult {
    pub status: UpdateStatus,
    pub error: Option<Error>,
    pub duration: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UpdateStatus {
    Completed,
    Failed,
    RequiresReboot,
}
```

### Update Record

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateRecord {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub update_type: UpdateType,
    pub package_name: String,
    pub old_version: String,
    pub new_version: String,
    pub status: UpdateStatus,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UpdateType {
    Package,
    Driver,
}
```


## Doğruluk Özellikleri (Correctness Properties)

Bir özellik (property), sistemin tüm geçerli çalıştırmalarında doğru olması gereken bir karakteristik veya davranıştır - esasen, sistemin ne yapması gerektiği hakkında resmi bir ifadedir. Özellikler, insan tarafından okunabilir spesifikasyonlar ile makine tarafından doğrulanabilir doğruluk garantileri arasında köprü görevi görür.

### Platform ve Tarama Özellikleri

**Özellik 1: Platform Tespiti**
*Her* uygulama başlatıldığında, tespit edilen platform gerçek çalışma platformu ile eşleşmelidir (Windows, macOS veya Linux)
**Doğrular: Gereksinim 1.1, 10.2**

**Özellik 2: Paket Tarama Bütünlüğü**
*Her* paket tarama işlemi için, döndürülen tüm paketler geçerli alanlara sahip olmalıdır (id, name, version, manager)
**Doğrular: Gereksinim 1.2**

**Özellik 3: Sürücü Tarama Bütünlüğü**
*Her* sürücü tarama işlemi için, döndürülen tüm sürücüler geçerli alanlara sahip olmalıdır (id, name, version, device_type)
**Doğrular: Gereksinim 1.3**

**Özellik 4: Tarama Performansı**
*Her* sistem tarama işlemi 30 saniye içinde tamamlanmalıdır
**Doğrular: Gereksinim 1.6**

### Güncelleme Kontrolü Özellikleri

**Özellik 5: Güncelleme Bilgisi Bütünlüğü**
*Her* bulunan güncelleme için, güncelleme bilgileri tüm gerekli alanları içermelidir (isim, mevcut versiyon, yeni versiyon)
**Doğrular: Gereksinim 2.4**

### Paket Güncelleme Özellikleri

**Özellik 6: Güncelleme Versiyon Değişimi**
*Her* başarılı paket güncellemesi için, paketin versiyonu güncelleme sonrası yeni versiyona değişmelidir
**Doğrular: Gereksinim 3.1**

**Özellik 7: Güncelleme Durum Takibi**
*Her* tamamlanan güncelleme için, güncelleme durumu "completed" olarak işaretlenmelidir
**Doğrular: Gereksinim 3.3**

**Özellik 8: Hata Kaydı**
*Her* başarısız güncelleme için, hata mesajı kaydedilmeli ve kullanıcıya gösterilmelidir
**Doğrular: Gereksinim 3.4**

### Sürücü Güncelleme Özellikleri

**Özellik 9: Sürücü Versiyon Değişimi**
*Her* başarılı sürücü güncellemesi için, sürücünün versiyonu güncelleme sonrası yeni versiyona değişmelidir
**Doğrular: Gereksinim 4.1**

**Özellik 10: Platform-Spesifik Sürücü Yönetimi**
*Her* platform için, sürücü güncellemeleri o platformun kendi sürücü sistemini kullanmalıdır (Windows: Windows Update API, macOS: System Updates, Linux: fwupd/kernel modules)
**Doğrular: Gereksinim 10.1, 10.2, 10.3, 10.5, 10.6, 10.7**

### Toplu Güncelleme Özellikleri

**Özellik 11: Öncelik Sıralaması**
*Her* toplu güncelleme için, güncellemeler öncelik sırasına göre (Critical > Important > Normal) uygulanmalıdır
**Doğrular: Gereksinim 5.5**

**Özellik 12: Toplu Güncelleme Özeti**
*Her* tamamlanan toplu güncelleme için, özet rapor başarılı, başarısız ve atlanan güncelleme sayılarını içermelidir
**Doğrular: Gereksinim 5.3**

### Güncelleme Geçmişi Özellikleri

**Özellik 13: Geçmiş Kayıt Bütünlüğü**
*Her* tamamlanan güncelleme için, kayıt tüm gerekli alanları içermelidir (tarih, paket adı, eski versiyon, yeni versiyon, durum)
**Doğrular: Gereksinim 6.2**

**Özellik 14: Geçmiş Sıralama**
*Her* geçmiş sorgusu için, sonuçlar tarih sırasına göre (en yeni önce) sıralanmalıdır
**Doğrular: Gereksinim 6.1**

**Özellik 15: Geçmiş Filtreleme**
*Her* filtrelenmiş geçmiş sorgusu için, tüm sonuçlar filtre kriterini karşılamalıdır
**Doğrular: Gereksinim 6.3**

**Özellik 16: Geçmiş Kalıcılığı (Round-trip)**
*Her* kaydedilen güncelleme kaydı için, veritabanından geri okunan kayıt orijinal kayıt ile eşdeğer olmalıdır
**Doğrular: Gereksinim 6.4**

### Otomatik Güncelleme Özellikleri

**Özellik 17: Ayar Kalıcılığı (Round-trip)**
*Her* otomatik güncelleme ayarı için, kaydedilip geri okunan ayar orijinal ayar ile eşdeğer olmalıdır
**Doğrular: Gereksinim 7.4**

### Sürücü Yedekleme Özellikleri

**Özellik 18: Otomatik Yedekleme**
*Her* sürücü güncellemesi başlatıldığında, güncelleme öncesi bir yedek oluşturulmalıdır
**Doğrular: Gereksinim 8.1**

**Özellik 19: Yedek Konum Kaydı**
*Her* oluşturulan yedek için, yedek dosyasının konumu kaydedilmelidir
**Doğrular: Gereksinim 8.2**

**Özellik 20: Sürücü Geri Yükleme (Round-trip)**
*Her* yedeklenen sürücü için, yedekten geri yükleme işlemi sürücüyü orijinal durumuna döndürmelidir
**Doğrular: Gereksinim 8.3**

**Özellik 21: Yedek Saklama Süresi**
*Her* yedek dosya için, dosya en az 30 gün boyunca saklanmalıdır
**Doğrular: Gereksinim 8.4**

### Paket Yöneticisi Entegrasyonu Özellikleri

**Özellik 22: Doğru Paket Yöneticisi Kullanımı**
*Her* paket güncellemesi için, güncelleme paketin ilişkili olduğu paket yöneticisini kullanmalıdır
**Doğrular: Gereksinim 9.4**

**Özellik 23: Paket Yöneticisi Görünürlüğü**
*Her* gösterilen paket için, paketin hangi paket yöneticisi tarafından yönetildiği bilgisi görünür olmalıdır
**Doğrular: Gereksinim 9.5**

**Özellik 24: Platform-Spesifik Komut Çalıştırma**
*Her* platform için, çalıştırılan paket yöneticisi komutları o platforma özgü olmalıdır
**Doğrular: Gereksinim 9.7**

### Çapraz Platform Özellikleri

**Özellik 25: Platform Tespit Doğruluğu**
*Her* uygulama başlatıldığında, OS_Detector tarafından tespit edilen platform ve versiyon gerçek çalışma ortamı ile eşleşmelidir
**Doğrular: Gereksinim 11.2**

**Özellik 26: Platform-Spesifik Özellik Görünürlüğü**
*Her* platform-spesifik özellik için, özellik sadece ilgili platformda görünür olmalıdır
**Doğrular: Gereksinim 11.5**

**Özellik 27: Veri Taşınabilirliği**
*Her* yapılandırma ve veri dosyası için, dosya tüm platformlarda okunabilir ve kullanılabilir olmalıdır
**Doğrular: Gereksinim 11.7**

**Özellik 28: Tauri 2.0 Native Performans**
*Her* platform için, Tauri_Runtime native performans sağlamalıdır (başlangıç süresi, bellek kullanımı)
**Doğrular: Gereksinim 11.8**

### Güvenlik Özellikleri

**Özellik 29: Checksum Doğrulama**
*Her* indirilen dosya için, dosyanın checksum'ı beklenen değer ile eşleşmelidir
**Doğrular: Gereksinim 12.2**

**Özellik 30: Güvenilir Kaynak Kontrolü**
*Her* güncelleme indirmesi için, kaynak yapılandırılmış güvenilir kaynaklar listesinde olmalıdır
**Doğrular: Gereksinim 12.3**

**Özellik 31: Hassas İşlem Loglama**
*Her* hassas işlem için, işlem güvenli bir şekilde loglanmalıdır
**Doğrular: Gereksinim 12.4**

**Özellik 32: Veri Şifreleme**
*Her* saklanan kullanıcı verisi için, veri şifrelenmiş formatta saklanmalıdır
**Doğrular: Gereksinim 12.5**

**Özellik 33: Tauri 2.0 Güvenlik**
*Her* IPC iletişimi için, Tauri 2.0 güvenlik özellikleri (IPC güvenliği, CSP) kullanılmalıdır
**Doğrular: Gereksinim 12.9**

### Kullanıcı Arayüzü Özellikleri

**Özellik 34: İlerleme Gösterimi**
*Her* uzun süren işlem için (tarama, güncelleme), UI ilerleme durumunu göstermelidir
**Doğrular: Gereksinim 1.5, 3.2, 4.3, 5.2**

**Özellik 35: Görsel Geri Bildirim**
*Her* kullanıcı işlemi başlatıldığında, UI anında görsel geri bildirim vermelidir
**Doğrular: Gereksinim 13.2**

**Özellik 36: Native Görünüm**
*Her* platform için, UI Tauri 2.0 webview kullanarak native görünüm ve his sunmalıdır
**Doğrular: Gereksinim 13.6, 13.7**

### Performans Özellikleri

**Özellik 37: Boşta Bellek Kullanımı**
*Her* boşta bekleyen uygulama durumu için, RAM kullanımı 100 MB'ın altında olmalıdır
**Doğrular: Gereksinim 14.1**

**Özellik 38: Tarama CPU Kullanımı**
*Her* tarama işlemi sırasında, CPU kullanımı %20'nin altında olmalıdır
**Doğrular: Gereksinim 14.2**

**Özellik 39: Asenkron İndirme**
*Her* indirme işlemi için, indirme UI'ı bloke etmemelidir
**Doğrular: Gereksinim 14.3**

**Özellik 40: Başlangıç Performansı**
*Her* uygulama başlatması için, başlangıç süresi 5 saniyenin altında olmalıdır
**Doğrular: Gereksinim 14.5**

**Özellik 41: Çapraz Platform Performans Tutarlılığı**
*Her* platform için, performans özellikleri (başlangıç, bellek, CPU) benzer olmalıdır
**Doğrular: Gereksinim 14.6, 14.7**

## Hata Yönetimi

### Hata Kategorileri

Uygulama, hataları şu kategorilere ayırır:

1. **Kritik Hatalar**: Uygulamanın çalışmasını engelleyen hatalar
   - Platform tespit edilemedi
   - Veritabanı bağlantısı başarısız
   - Yetki yükseltme reddedildi

2. **Önemli Hatalar**: İşlemin başarısız olmasına neden olan hatalar
   - Güncelleme indirme başarısız
   - Checksum doğrulama başarısız
   - Sürücü yedekleme başarısız

3. **Normal Hatalar**: Kullanıcıya bildirilen ancak işlemi durdurmayan hatalar
   - İnternet bağlantısı yok
   - Paket yöneticisi bulunamadı
   - Disk alanı yetersiz

### Hata İşleme Stratejileri

#### Network Hataları

```rust
async fn handle_network_error(&self, error: NetworkError) -> Result<()> {
    match error {
        NetworkError::NoConnection => {
            // Kullanıcıya bildir
            self.notification_service.show_error(
                "İnternet bağlantısı yok. Lütfen bağlantınızı kontrol edin."
            ).await;
            
            // Offline moda geç
            self.set_offline_mode(true).await;
        }
        NetworkError::Timeout => {
            // Retry mekanizması
            self.retry_with_backoff().await?;
        }
        NetworkError::ServerError(code) => {
            // Sunucu hatası logla
            error!("Server error: {}", code);
            self.notification_service.show_error(
                "Güncelleme sunucusuna erişilemiyor."
            ).await;
        }
    }
    Ok(())
}
```

#### Güncelleme Hataları

```rust
async fn handle_update_error(&self, update: &Update, error: UpdateError) -> Result<()> {
    match error {
        UpdateError::ChecksumMismatch => {
            // Güvenlik hatası - güncellemeyi iptal et
            error!("Checksum mismatch for update: {:?}", update);
            self.notification_service.show_error(
                "Güncelleme dosyası doğrulanamadı. Güvenlik nedeniyle iptal edildi."
            ).await;
            
            // Hatalı dosyayı sil
            self.cleanup_failed_download(update).await?;
        }
        UpdateError::InsufficientSpace => {
            // Disk alanı yetersiz
            self.notification_service.show_error(
                "Yetersiz disk alanı. Lütfen yer açın ve tekrar deneyin."
            ).await;
            
            // Eski yedekleri temizleme öner
            self.suggest_cleanup().await?;
        }
        UpdateError::PermissionDenied => {
            // Yetki hatası
            self.notification_service.show_error(
                "Güncelleme için yönetici yetkisi gerekiyor."
            ).await;
            
            // Yetki yükseltme iste
            self.platform_adapter.request_elevation().await?;
        }
    }
    Ok(())
}
```

#### Sürücü Hataları

```rust
async fn handle_driver_error(&self, driver: &Driver, error: DriverError) -> Result<()> {
    match error {
        DriverError::UpdateFailed => {
            // Sürücü güncellemesi başarısız - geri yükle
            warn!("Driver update failed, rolling back: {:?}", driver);
            
            if let Some(backup_path) = self.get_backup_path(driver).await? {
                self.platform_adapter.restore_driver(&backup_path).await?;
                
                self.notification_service.show_info(
                    "Sürücü güncellemesi başarısız oldu. Önceki sürüm geri yüklendi."
                ).await;
            }
        }
        DriverError::BackupFailed => {
            // Yedekleme başarısız - güncellemeyi iptal et
            error!("Driver backup failed: {:?}", driver);
            self.notification_service.show_error(
                "Sürücü yedeklenemedi. Güvenlik nedeniyle güncelleme iptal edildi."
            ).await;
        }
        DriverError::RestoreFailed => {
            // Geri yükleme başarısız - kritik durum
            error!("Driver restore failed: {:?}", driver);
            self.notification_service.show_error(
                "Sürücü geri yüklenemedi. Lütfen manuel olarak geri yükleyin."
            ).await;
            
            // Yedek konumunu göster
            if let Some(backup_path) = self.get_backup_path(driver).await? {
                self.notification_service.show_info(
                    &format!("Yedek konumu: {}", backup_path.display())
                ).await;
            }
        }
    }
    Ok(())
}
```

### Hata Loglama

Tüm hatalar yapılandırılabilir log seviyelerine göre loglanır:

```rust
pub enum LogLevel {
    Error,   // Kritik hatalar
    Warn,    // Önemli hatalar
    Info,    // Bilgilendirme mesajları
    Debug,   // Debug bilgileri
}

impl ErrorHandler {
    pub async fn log_error(&self, error: &Error, level: LogLevel) {
        let log_entry = LogEntry {
            timestamp: Utc::now(),
            level,
            error_type: error.error_type(),
            message: error.to_string(),
            stack_trace: error.backtrace(),
            context: self.get_context(),
        };
        
        // Dosyaya yaz
        self.log_writer.write(&log_entry).await;
        
        // Kritik hatalarda kullanıcıya bildir
        if level == LogLevel::Error {
            self.notification_service.show_error(&error.user_message()).await;
        }
    }
}
```

## Test Stratejisi

### İkili Test Yaklaşımı

Uygulama, kapsamlı test kapsamı için hem unit testler hem de property-based testler kullanır:

- **Unit Testler**: Spesifik örnekler, edge case'ler ve hata durumları
- **Property Testler**: Tüm girdiler üzerinde evrensel özellikler

Her iki test türü de birbirini tamamlar ve birlikte kapsamlı kapsam sağlar.

### Property-Based Testing

Property-based testler için **proptest** (Rust) kütüphanesi kullanılır. Her test minimum 100 iterasyon çalıştırılır.

#### Test Yapılandırması

```rust
use proptest::prelude::*;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]
    
    // Feature: system-update-manager, Property 1: Platform Tespiti
    #[test]
    fn test_platform_detection(platform in any::<Platform>()) {
        let adapter = create_adapter_for_platform(&platform);
        let detected = adapter.detect_platform();
        prop_assert_eq!(detected, platform);
    }
}
```

#### Örnek Property Testler

**Özellik 6: Güncelleme Versiyon Değişimi**

```rust
proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]
    
    // Feature: system-update-manager, Property 6: Güncelleme Versiyon Değişimi
    #[test]
    fn test_update_changes_version(
        package in arb_package(),
        new_version in arb_version()
    ) {
        let update = PackageUpdate {
            package: package.clone(),
            new_version: new_version.clone(),
            ..Default::default()
        };
        
        let result = apply_update(&update).await?;
        prop_assert_eq!(result.status, UpdateStatus::Completed);
        
        let updated_package = get_package(&package.id).await?;
        prop_assert_eq!(updated_package.version, new_version);
    }
}
```

**Özellik 11: Öncelik Sıralaması**

```rust
proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]
    
    // Feature: system-update-manager, Property 11: Öncelik Sıralaması
    #[test]
    fn test_priority_ordering(updates in prop::collection::vec(arb_update(), 1..20)) {
        let sorted = sort_by_priority(updates.clone());
        
        // Tüm ardışık çiftleri kontrol et
        for window in sorted.windows(2) {
            prop_assert!(window[0].priority >= window[1].priority);
        }
    }
}
```

**Özellik 16: Geçmiş Kalıcılığı (Round-trip)**

```rust
proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]
    
    // Feature: system-update-manager, Property 16: Geçmiş Kalıcılığı
    #[test]
    fn test_history_persistence_roundtrip(record in arb_update_record()) {
        // Kaydet
        history_manager.save_record(&record).await?;
        
        // Geri oku
        let retrieved = history_manager.get_record(&record.id).await?;
        
        // Eşdeğer olmalı
        prop_assert_eq!(retrieved, record);
    }
}
```

**Özellik 29: Checksum Doğrulama**

```rust
proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]
    
    // Feature: system-update-manager, Property 29: Checksum Doğrulama
    #[test]
    fn test_checksum_validation(
        file_content in prop::collection::vec(any::<u8>(), 1..1024),
        expected_checksum in arb_checksum()
    ) {
        let actual_checksum = calculate_checksum(&file_content);
        
        if actual_checksum == expected_checksum {
            prop_assert!(validate_checksum(&file_content, &expected_checksum).is_ok());
        } else {
            prop_assert!(validate_checksum(&file_content, &expected_checksum).is_err());
        }
    }
}
```

### Unit Testing

Unit testler spesifik senaryoları ve edge case'leri test eder:

#### Platform-Spesifik Testler

```rust
#[cfg(target_os = "windows")]
#[test]
fn test_windows_package_managers() {
    let adapter = WindowsAdapter::new();
    let managers = adapter.get_package_managers();
    
    assert!(managers.contains(&PackageManager::Winget));
}

#[cfg(target_os = "macos")]
#[test]
fn test_macos_package_managers() {
    let adapter = MacOSAdapter::new();
    let managers = adapter.get_package_managers();
    
    assert!(managers.contains(&PackageManager::Homebrew) || 
            managers.contains(&PackageManager::MacPorts));
}

#[cfg(target_os = "linux")]
#[test]
fn test_linux_package_managers() {
    let adapter = LinuxAdapter::new();
    let managers = adapter.get_package_managers();
    
    assert!(!managers.is_empty());
}
```

#### Hata Durumu Testleri

```rust
#[tokio::test]
async fn test_no_internet_connection() {
    let scanner = create_scanner_with_no_network();
    let result = scanner.check_updates(&scan_result).await;
    
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), ErrorKind::NoConnection);
}

#[tokio::test]
async fn test_insufficient_disk_space() {
    let engine = create_engine_with_low_disk_space();
    let result = engine.apply_update(&update).await;
    
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), ErrorKind::InsufficientSpace);
}
```

#### Edge Case Testleri

```rust
#[test]
fn test_empty_package_list() {
    let packages = vec![];
    let updates = check_updates(&packages);
    
    assert!(updates.is_empty());
}

#[test]
fn test_driver_backup_with_no_space() {
    let backup_manager = create_backup_manager_with_no_space();
    let result = backup_manager.backup_driver(&driver).await;
    
    assert!(result.is_err());
}
```

### Integration Testing

Integration testler, bileşenler arası etkileşimleri test eder:

```rust
#[tokio::test]
async fn test_full_update_workflow() {
    // Sistem tarama
    let scan_result = scanner.scan_system().await.unwrap();
    assert!(!scan_result.packages.is_empty());
    
    // Güncelleme kontrolü
    let updates = scanner.check_updates(&scan_result).await.unwrap();
    
    if !updates.is_empty() {
        // İlk güncellemeyi uygula
        let update = &updates[0];
        let result = engine.apply_update(update).await.unwrap();
        
        assert_eq!(result.status, UpdateStatus::Completed);
        
        // Geçmişte kayıtlı olmalı
        let history = history_manager.get_history(HistoryFilter::All).await.unwrap();
        assert!(!history.is_empty());
    }
}
```

### Test Kapsamı Hedefleri

- **Backend Rust Kodu**: Minimum %80 kod kapsamı
- **Frontend TypeScript Kodu**: Minimum %70 kod kapsamı
- **Kritik Yollar**: %100 kapsam (güvenlik, veri bütünlüğü)
- **Property Testler**: Her doğruluk özelliği için bir test

### CI/CD Pipeline

Testler, her commit'te otomatik olarak çalıştırılır:

```yaml
test:
  stage: test
  script:
    # Unit ve property testleri
    - cargo test --all-features
    
    # Integration testleri
    - cargo test --test integration_tests
    
    # Frontend testleri
    - npm test
    
    # Kapsam raporu
    - cargo tarpaulin --out Xml
```

