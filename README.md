# Dexo

Dexo adalah aplikasi desktop download manager modern yang dibangun sebagai alternatif Internet Download Manager (IDM) dengan performa tinggi berbasis **Rust + Tauri 2** dan **Svelte + TypeScript + Tailwind CSS**.

## ✨ Fitur Utama (MVP)
- **Multi-connection Acceleration**: Membagi file menjadi beberapa segmen byte simultan untuk memaksimalkan throughput internet.
- **Tampilan Klasik Modern IDM**: Menu Bar, Action Toolbar berdensitas tinggi, Category Tree, dan Transfer Grid yang presisi.
- **Ekstensi Browser (Manifest V3)**: Otomatis menangkap tautan unduhan dari Chrome, Edge, Brave, dan Google Drive.
- **Probe Metadata Otomatis**: Mendeteksi ukuran file, nama file dari header Content-Disposition, dan dukungan HTTP Range.
- **Pause & Resume**: Melanjutkan unduhan tanpa mengulang dari awal.
- **Persistensi SQLite**: Riwayat dan status unduhan disimpan di database lokal.

## 🚀 Menjalankan Proyek
### Prasyarat
1. **Node.js** (v18+)
2. **Rust** toolchain:
   Unduh installer Rust dari [https://rustup.rs](https://rustup.rs) atau `winget install Rustlang.Rustup`

### Instalasi & Menjalankan Dev Mode
```powershell
# Install dependensi frontend
npm install

# Jalankan dalam mode desktop dev
npm run tauri dev
```
