## Modul 3: Berbagi State & Manajemen Konfigurasi!


## Topik 1: Berbagi *State* Sederhana dengan `Mutex`

Bayangkan kita ingin membuat penghitung jumlah pengunjung di situs kita. Setiap kali seseorang mengunjungi halaman utama, angkanya akan bertambah. Ini berarti kita butuh sebuah data (angka counter) yang bisa **diakses dan diubah** oleh setiap request yang masuk.

Data seperti ini disebut **Application State**.

### Masalah: *Race Condition*

Jika dua pengunjung datang di waktu yang *persis* bersamaan, kedua *request* bisa saja sama-sama membaca angka counter (misalnya 5), sama-sama menambahkannya menjadi 6, lalu menyimpannya. Hasil akhirnya 6, padahal seharusnya 7. Ini disebut *race condition*.

### Solusi: `Mutex` (Mutual Exclusion)

Untuk mencegah hal ini, Rust menyediakan `Mutex`. Anggap saja `Mutex` ini seperti **kunci toilet di kantor**.

  * Siapa pun yang mau memakai toilet (mengubah data), harus ambil kuncinya dulu (`.lock()`).
  * Selama kuncinya dipegang, orang lain harus antre dan menunggu.
  * Setelah selesai, kuncinya dikembalikan secara otomatis, dan orang berikutnya boleh masuk.

Ini memastikan hanya ada satu proses yang bisa mengubah data pada satu waktu.

-----

### Implementasi di Actix

Kita akan menggunakan `Mutex` untuk membungkus data counter kita dan `web::Data` untuk membagikannya ke semua *handler*.

**Langkah 1: Definisikan `AppState`**

Kita buat sebuah `struct` untuk menampung *state* aplikasi kita.

```rust
use std::sync::Mutex;

struct AppState {
    counter: Mutex<usize>, // usize adalah tipe data angka positif
}
```

**Langkah 2: Daftarkan `AppState` di `main`**

Di fungsi `main`, kita akan membuat *state*-nya dan mendaftarkannya ke aplikasi menggunakan `.app_data()`.

```rust
// main.rs
use actix_web::{get, web, App, HttpServer, Responder};
use std::sync::Mutex;

// ... definisi struct AppState di sini ...

#[get("/")]
async fn index(data: web::Data<AppState>) -> String {
    // 1. Kunci Mutex untuk mendapatkan akses eksklusif
    let mut counter = data.counter.lock().unwrap();
    // 2. Ubah data (dereference dengan *)
    *counter += 1;
    // 3. Buat respons. Lock otomatis dilepas saat fungsi selesai.
    format!("Anda adalah pengunjung ke-{}", *counter)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Buat state yang akan dibagikan
    let app_state = web::Data::new(AppState {
        counter: Mutex::new(0), // Mulai dari 0
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone()) // Daftarkan state ke aplikasi
            .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
```

**Penjelasan Kode:**

  * `web::Data::new(...)`: Membungkus *state* kita ke dalam `web::Data`. Ini adalah cara Actix untuk bisa berbagi data antar *thread* dengan aman.
  * `.app_data(...)`: Mendaftarkan `web::Data` tersebut ke aplikasi. Semua *handler* di dalam `App` ini sekarang bisa memintanya.
  * `data: web::Data<AppState>`: Di dalam *handler*, kita menggunakan ini sebagai *extractor* untuk mendapatkan akses ke *state* yang sudah kita daftarkan.
  * `.lock().unwrap()`: Ini adalah cara kita "meminta kunci" `Mutex`.
  * `*counter += 1`: Tanda `*` (dereference) kita gunakan untuk mengakses dan mengubah nilai `usize` yang ada di dalam `Mutex`.

Sekarang, setiap kali Anda me-refresh halaman utama, angkanya akan terus bertambah dengan aman.

-----

### **Materi & Dokumentasi**

  * **Application State:** Dokumentasi resmi Actix tentang berbagi state.
      * [Shared State - Actix Docs](https://www.google.com/search?q=https://actix.rs/docs/application%23shared-state)
  * **`std::sync::Mutex`:** Dokumentasi Rust tentang Mutex.
      * [Mutex in std::sync - Rust Docs](https://doc.rust-lang.org/std/sync/struct.Mutex.html)


-----

Mari kita lanjutkan ke bagian kedua dari Modul 3: **Manajemen Konfigurasi**.

### **Masalah: *Hardcoding* Itu Berbahaya**

Selama ini, kita menulis alamat server dan port langsung di dalam kode:
`  .bind(("127.0.0.1", 8080))? `

Ini disebut *hardcoding*. Mengapa ini praktik yang buruk?

  * **Tidak Fleksibel:** Bagaimana jika saat di-upload ke server (produksi), port `8080` sudah dipakai? Kita harus mengubah kode dan kompilasi ulang.
  * **Tidak Aman:** Kita tidak boleh menyimpan informasi sensitif seperti *password database* atau *API key* langsung di dalam kode.

### **Solusi: Muat Konfigurasi dari Lingkungan (*Environment*)**

Praktik terbaik adalah memisahkan konfigurasi dari kode. Salah satu cara paling umum adalah menggunakan file `.env` untuk menyimpan konfigurasi saat pengembangan lokal.

Kita akan menggunakan sebuah *crate* (library) populer bernama `dotenvy` untuk membantu kita.

-----

### **Implementasi**

**Langkah 1: Tambahkan `dotenvy` ke `Cargo.toml`**

```toml
[dependencies]
actix-web = "4"
serde = { version = "1.0", features = ["derive"] }
std_sync_mutex = "0.0.0" # ini sepertinya tidak perlu, std::sync::Mutex sudah bawaan
dotenvy = "0.15" # Tambahkan ini
```

*(Catatan: `std::sync::Mutex` adalah bagian dari library standar Rust, jadi Anda tidak perlu menambahkannya ke `Cargo.toml`)*

**Langkah 2: Buat File `.env`**

Di **direktori utama** proyek Anda (sejajar dengan `Cargo.toml`), buat sebuah file baru bernama `.env` (diawali dengan titik).

**Isi file `.env`:**

```
HOST=127.0.0.1
PORT=8080
```

**Langkah 3: Muat dan Gunakan di `main.rs`**

Sekarang, kita modifikasi `main.rs` untuk membaca variabel dari file `.env` tersebut.

```rust
use actix_web::{web, App, HttpServer};
use std::env; // Modul untuk mengakses environment variables

// ... use statements dan module declarations lainnya ...

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 1. Muat variabel dari file .env di awal sekali
    dotenvy::dotenv().ok();

    // 2. Baca variabel dari environment
    let host = env::var("HOST").expect("HOST harus diset di file .env");
    let port_str = env::var("PORT").expect("PORT harus diset di file .env");
    // .parse() mengubah String menjadi tipe lain, dalam hal ini u16 (angka untuk port)
    let port = port_str.parse::<u16>().expect("PORT harus berupa angka");

    println!("🚀 Menjalankan server di http://{}:{}", host, port);

    HttpServer::new(move || {
        App::new()
            // ... service dan app_data Anda ...
    })
    .bind((host, port))? // 3. Gunakan variabel di sini
    .run()
    .await
}
```

**Penjelasan Kode:**

  * `dotenvy::dotenv().ok();`: Perintah ini mencari file `.env` di direktori proyek dan memuat semua variabel di dalamnya ke *environment*. `.ok()` membuatnya tidak akan *crash* jika file `.env` tidak ditemukan (berguna untuk lingkungan produksi).
  * `env::var("NAMA_VARIABEL")`: Fungsi ini membaca nilai dari *environment*. Ia mengembalikan `Result`, jadi kita pakai `.expect()` untuk mengambil nilainya atau *crash* dengan pesan error jika tidak ditemukan.
  * `port_str.parse::<u16>()`: Karena `env::var` selalu menghasilkan `String`, kita perlu mengubahnya (`parse`) menjadi tipe angka `u16` yang dibutuhkan oleh fungsi `.bind()`.

Sekarang, jika Anda perlu mengubah port, Anda cukup mengubahnya di file `.env` tanpa menyentuh kode Rust sama sekali\!

-----

### **Materi & Dokumentasi**

  * **`dotenvy` crate:** Halaman resmi `dotenvy` untuk informasi lebih lanjut.
      * [`dotenvy` on crates.io](https://www.google.com/search?q=%5Bhttps://crates.io/crates/dotenvy%5D\(https://crates.io/crates/dotenvy\))



-----
## **📝 Notes**
Selanjutnya, kita akan memasuki salah satu modul paling krusial untuk membangun aplikasi nyata:
Modul 3: Berbagi State & Manajemen Konfigurasi
