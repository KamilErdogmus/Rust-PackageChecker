# Package Checker UI Redesign Plan

## 1. Yönetici Özeti (Executive Summary)

Mevcut UI, "yapay zeka üretimi" gibi hissettiren veya temel bir demo uygulamasını andıran standart, işlenmemiş bileşenlerin bir araya gelmesinden oluşuyor. Hedefimiz: Uygulamayı modern bir "macOS Sistem Ayarları" veya "Linear / Vercel Dashboard" seviyesinde profesyonel, temiz, insan yapımı ve yüksek kullanıcı deneyimine sahip bir hale getirmektir.

## 2. Genel Konsept ve Düzen (Layout)

Mevcut yapı, tepeye sabitlenmiş bir "Header" ve altında "Card" yığınından oluşuyor. Bunu değiştirmeli ve daha profesyonel bir düzene geçmeliyiz.

- **Önerilen Düzen:** İki sütunlu bir yapı (Sidebar + Main Content).
  - **Sol Sidebar (Navigation):** Profil/Log, "Genel Bakış" (Overview), "Güncellemeler" (Updates), "Geçmiş" (History), ve "Ayarlar/Yoksayılanlar" (Settings/Ignored) sekmeleri yer alacak.
  - **Sağ İçerik Alanı:** Seçilen menüye ait içerik burada yüklenecek. Maksimum genişlik sınırlandırılarak okunaklılık artırılacak.

## 3. Renk Paleti ve Temalama

`src/index.css` dosyasındaki mevcut HSL değerleri (Mavi tonlu standart shadcn paleti) biraz fazla "varsayılan" hissettiriyor.

- **Birincil Renk (Primary):** Şık, pastel veya hafif grileştirilmiş bir ton (örneğin Zinc veya Neutral temelli monokrom bir primary, veya Tailwind'deki `indigo-500`/`blue-500` tonlarına benzer daha soft bir renk).
- **Yüzey ve Arka Plan (Background/Card):**
  - Dark modda arka plan simsiyah (`bg-black`) veya çok koyu gri (`bg-zinc-950`), kartlar ise bir tık açık (`bg-zinc-900`) olmalı ve sadece çok hafif bir `border-white/5` çizgisiyle ayrılmalı.
  - Light modda ise arka plan tamamen beyaz (`bg-white`), kartlar çok hafif gri (`bg-zinc-50` / `bg-slate-50`) olmalı.
- **Radius:** `--radius` değişkeni şu an `0.5rem`. Bu değer günümüz standartları için iyi, ancak butonlar ve inputlar için daha yuvarlak veya daha keskin hatlar (örneğin `0.3rem` veya `0.75rem`) projenin stiline göre seçilmeli.

## 4. Tipografi ve İkonografi

- **Tipografi:** Varsayılan font yerine `Geist`, `Inter` veya `SF Pro` gibi modern sans-serif fontlar projeye entegre edilmeli. (Örn: `@fontsource/inter`). Başlıklar (h1, h2) biraz daha sıkı (`tracking-tight`) olmalı.
- **İkonografi:** `lucide-react` kullanılmaya devam edilecek, ancak ikon boyutları, yanındaki metinlerle görsel olarak tam hizalanacak şekilde ayarlanmalı (örn. `w-4 h-4 mr-2`). İkonların strok kalınlıkları (stroke-width) UI genelinde tutarlı olmalı.

## 5. UI Bileşenleri (shadcn/ui ve Tailwind)

Mevcut uygulamada `<select>` etiketleri (native) kullanılıyor ve bu çok amatör bir görüntü yaratıyor.
Gereken Yeni Bileşenler (shadcn/ui üzerinden eklenebilir):

- **Command / Command Palette:** Paket araması için standart `<input>` yerine klavye kısayolu destekleyen (`Ctrl+K` / `Cmd+K`) şık bir command palette arayüzü eklenebilir.
- **Select / DropdownMenu:** Kategori filtresi ve Sıralama (Sort) işlemleri için native `<select>` yerine özel tasarım `Select` veya `DropdownMenu` kullanılmalı.
- **Table / Data Table:** "Updates" listesi ayrı ayrı kartlar yerine, şık bir veri tablosunda listelenmeli. "Paket Adı", "Kategori", "Mevcut Sürüm", "Yeni Sürüm" ve "Aksiyonlar" sütunları olmalı.
- **Tabs:** Eğer Sidebar yapısına geçilmeyecekse, içerikte gezinmek için "Güncellemeler", "Geçmiş", "Yoksayılanlar" gibi `Tabs` bileşeni kullanılmalı.
- **Progress / Skeleton:** Arama (Scan) veya Güncelleme sırasında tüm sayfayı dondurmak yerine, Shimmer animasyonlu `Skeleton` veya güzel bir `Progress` çubuğu gösterilmeli.
- **Toast / Sonner:** "X paketi başarıyla güncellendi" gibi mesajlar için ekranın üstünde/altında çıkan çirkin uyarı kutuları yerine `sonner` veya `toast` bildirimleri kullanılmalı.

## 6. Ekran Ekran Tasarım Detayları

1. **Genel Bakış (Overview/Dashboard):**
   - En tepede "Sisteminiz Güncel" veya "3 Güncelleme Bekliyor" yazan büyük bir hero alanı.
   - İstatistiklerin (`Total Packages`, `Updates Available`) gösterildiği grid tarzı şık kartlar (Mevcut yapı fena değil ama ikonları vurgulamak için daha fazla negatif alan verilmeli).
2. **Güncelleme Listesi (Updates):**
   - Üst kısımda "Filtreleme Çubuğu" (Arama kutusu, Kategori Select'i, Sort Select'i).
   - Altında güncellenecek uygulamaların tablosu. Tablo satırına "hover" olunduğunda ilgili uygulamanın logosu/ikonu hafif belirmeli veya renk değiştirmeli.
   - Çoklu seçim için modern checkbox'lar kullanılmalı.
3. **Güncelleniyor Modülü (Active Action):**
   - "Şu an güncelleniyor" kısmı, basit bir alert yerine sayfanın altında kalıcı olarak duran, iptal edilebilen (veya süreci gösteren) şık bir Dock / Floating Notification olarak tasarlanmalı.

## 7. Mimari Geçiş Adımları

1. **Bağımlılıkların Eklenmesi:** `shadcn-ui/ui` CLI ile eksik bileşenlerin (Table, Select, Tabs, DropdownMenu, Skeleton) eklenmesi.
2. **Düzenin (Layout) Değiştirilmesi:** `App.tsx` dosyasının, ana Layout ve Alt Sayfalar/Bileşenler olarak (Sidebar/Header + Content) refactor edilmesi.
3. **Bileşen Yenileme:** Native elementlerin (select, checkbox) kaldırılıp UI bileşenleriyle değiştirilmesi.
4. **Stil Optimizasyonu:** `tailwind.config.js` ve `index.css` üzerinde renk paletinin güncellenmesi.
5. **Animasyonlar:** `framer-motion` veya Tailwind `animate-in` ile yumuşak geçişlerin sağlanması.
