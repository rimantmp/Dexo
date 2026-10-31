# Downbyte Integration Module (Browser Extension)

Ekstensi browser resmi Downbyte yang dirancang dengan **Manifest V3** untuk menangkap dan mengalihkan unduhan dari browser langsung ke aplikasi desktop **Downbyte**.

## 🌟 Fitur Utama
- **Automatic Download Interception**: Otomatis menangkap tautan unduhan sesuai format file yang dikonfigurasi (`.zip`, `.rar`, `.exe`, `.mp4`, `.pdf`, dll.) dan menghentikan pengunduhan lambat di browser.
- **Video Sniffer Panel**: Menampilkan tombol mengambang *"Download with Downbyte"* di atas elemen video pada halaman web (mirip fitur video download panel pada IDM).
- **Context Menu Integration**: Klik kanan pada tautan, gambar, atau media apa pun lalu pilih **"Download with Downbyte"**.
- **Quick Download Popup**: Paste URL langsung di popup ekstensi untuk memulai download instan.

---

## 🛠️ Cara Memasang Ekstensi di Browser

### 1. Google Chrome / Microsoft Edge / Brave
1. Buka browser dan kunjungi halaman ekstensi:
   - Chrome / Brave: `chrome://extensions`
   - Edge: `edge://extensions`
2. Aktifkan **Developer mode** (Mode Pengembang) di pojok kanan atas.
3. Klik tombol **Load unpacked** (Muat yang belum dibongkar).
4. Pilih folder:
   ```text
   D:\PROJEK\01\browser-extension
   ```
5. Ekstensi **Downbyte Integration Module** akan langsung aktif dan muncul di toolbar browser Anda.

---

### 2. Registrasi Native Messaging Host (Opsional untuk integrasi Windows native)
Untuk mengizinkan ekstensi berkomunikasi langsung dengan desktop Downbyte lewat protokol standar Windows:
1. Masuk ke folder `browser-extension/native-host`.
2. Klik kanan file `install-host.bat` dan pilih **Run as Administrator** (atau jalankan langsung).
3. Registry untuk Chrome dan Edge akan otomatis terpasang.

