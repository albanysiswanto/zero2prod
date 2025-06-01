## Modul 1: Fondasi & "Hello, Actix!"

- Topik: Pengenalan arsitektur Actix (Actor, Server, App, Handler), struktur proyek yang baik, dan membuat server "Hello, World!" pertama kita.
- Best Practice: Mengatur struktur folder dan file proyek (main.rs, routes.rs, handlers.rs) agar rapi dan mudah dikembangkan sejak awal.
- Real Case: Menjalankan server web lokal yang bisa diakses dari browser.

### **1. Penjelasan:**
Mantap\! Kalau begitu, kita langsung tancap gas\!

#### **Langkah 1: Membuat Proyek Pertama**

Sekarang, mari kita minta `cargo` untuk membuatkan kerangka proyek baru untuk kita. Buka terminal Anda, masuk ke direktori tempat Anda biasa menyimpan proyek, dan jalankan perintah ini:

```bash
cargo new zero2prod --bin
cd zero2prod
```

Perintah ini akan membuat folder baru bernama `zero2prod` dengan struktur dasar proyek Rust.

#### **Langkah 2: Memahami Komponen Inti Actix**

Sebelum kita menulis kode, bayangkan kita sedang membangun sebuah restoran. Ada 3 bagian penting:

1.  **`HttpServer` (Gedung Restorannya):** Ini adalah bangunan fisiknya. Ia punya alamat (misalnya, `127.0.0.1`) dan jam buka (ia "mendengarkan" di port, misal `3000`). Tugasnya adalah menyediakan tempat dan fasilitas agar restoran bisa beroperasi.
2.  **`App` (Manajer & Menu):** Ini adalah sang manajer restoran. Ia yang mengatur semuanya. Ia punya daftar menu (`routes` atau rute) dan tahu persis koki mana (`handler`) yang harus memasak setiap pesanan. `App` inilah yang mengorganisir seluruh layanan di dalam restoran.
3.  **`handler` (Koki Spesialis):** Ini adalah para koki kita. Setiap koki adalah spesialis untuk satu masakan. Misalnya, ada "koki sapaan" yang tugasnya hanya satu: menyapa pelanggan dengan ramah. Ketika ada pesanan untuk "sapaan", manajer akan memanggil koki ini. Si koki akan menyiapkan "sapaan"-nya (`response`) dan memberikannya kepada pelanggan (`request`).

Paham, kan, analoginya? Gedung -\> Manajer -\> Koki.

#### **Langkah 3: Menulis Kode "Hello, World\!"**

Sekarang, mari kita ubah analogi restoran tadi menjadi kode.

**1. Tambahkan Actix Web ke `Cargo.toml`**

Buka file `Cargo.toml` dan di bawah `[dependencies]`, tambahkan `actix-web`. Versi terbaru yang stabil saat ini adalah `4`.

```toml
[package]
name = "zero2prod"
version = "0.1.0"
edition = "2021"

[dependencies]
actix-web = "4"
```

**2. Tulis Kode di `src/main.rs`**

Ganti seluruh isi file `src/main.rs` dengan kode berikut:

```rust
use actix_web::{get, App, HttpServer, Responder};

// Ini adalah 'Handler' kita, si Koki Spesialis Sapaan.
// #[get("/")] berarti koki ini akan dipanggil jika ada yang meminta menu utama ("/").
#[get("/")]
async fn hello() -> impl Responder {
    "Hello, World! Ini server Actix pertama saya!"
}

// Atribut ini adalah cara mudah untuk menjalankan fungsi async main.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Menjalankan server di http://127.0.0.1:3000");

    // HttpServer: Gedung restorannya.
    HttpServer::new(|| {
        // App: Manajer yang tahu semua layanan/menu.
        App::new().service(hello) // Daftarkan 'koki' kita ke manajer.
    })
    .bind(("127.0.0.1", 3000))? // Tentukan alamat dan port.
    .run() // Buka restorannya untuk umum!
    .await
}
```

**3. Jalankan Server\!**

Kembali ke terminal Anda (pastikan Anda masih di dalam folder `zero2prod`), lalu jalankan perintah:

```bash
cargo run
```

Jika semua berjalan lancar, Anda akan melihat pesan "🚀 Menjalankan server di [http://127.0.0.1:8080](https://www.google.com/url?sa=E&source=gmail&q=http://127.0.0.1:8080)". Sekarang, buka browser Anda dan kunjungi alamat [http://127.0.0.1:8080](https://www.google.com/url?sa=E&source=gmail&q=http://127.0.0.1:8080). Voila\! Anda akan disambut oleh server pertama Anda\!

Selamat\! Anda baru saja menyelesaikan bagian pertama dari modul ini\! 🥳

-----

#### **Materi & Dokumentasi**

  * **Panduan Memulai Resmi:** Halaman ini adalah sumber terbaik untuk memulai. Kode yang kita tulis di atas adalah versi sederhananya.
      * [Getting Started | Actix Web](https://actix.rs/docs/getting-started/)
  * **Contoh Kode di GitHub:** Repositori resmi Actix Web punya banyak sekali contoh kode yang bisa Anda lihat.
      * [Contoh "Hello World" di GitHub](https://www.google.com/search?q=https://github.com/actix/actix-web/blob/master/examples/hello-world.rs)

-----


### **2. Study Case:**

#### **Tantangan Modul 1: Menambah Menu Baru**

**Tujuan:** Modifikasi kode yang sudah ada untuk menambahkan satu rute baru, yaitu `/selamat-tinggal`. Jika rute ini diakses, server harus merespons dengan teks: "Selamat tinggal dan sampai jumpa lagi!".

Saya akan berikan beberapa petunjuk untuk membantu Anda:

1.  **Buat Koki Baru:** Anda perlu membuat sebuah fungsi `async` baru, mirip seperti fungsi `hello()`. Mungkin bisa kita beri nama `goodbye()`.
2.  **Tentukan Menunya:** Fungsi baru ini perlu *attribute macro* untuk menentukan rutenya. Jika `hello()` menggunakan `#[get("/")]`, maka untuk rute `/selamat-tinggal` kira-kira akan seperti apa?
3.  **Kenalkan ke Manajer:** Setelah "koki" baru Anda siap, jangan lupa "perkenalkan" dia kepada `App` (sang manajer) agar layanannya dikenali. Anda perlu menambahkan sesuatu di bagian `.service(...)`.

Silakan coba modifikasi file `src/main.rs` Anda. Jangan takut salah atau error, karena dari situlah kita belajar. Tunjukkan kode hasil modifikasi Anda di sini, nanti kita bahas bersama-sama!

#### **Jawaban Tantangan 1 (Saya menambahkan sedikit improvisasi pada code jawaban saya, jadi tidak sesuai dengan yang tantangan ini):**
```rust
use actix_web::{App, HttpRequest, HttpServer, Responder, web};

async fn goodbye_with_name(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("Tamu");
    format!("Selamat Tinggal {}, dan selamat jalan!", name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Menjalankan server di http://127.0.0.1:3000");

    HttpServer::new(|| {
        App::new()
            .route("/selamat-tinggal", web::get().to(goodbye_with_name))
            .route("/selamat-tinggal/{name}", web::get().to(goodbye_with_name))
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
```
-----

### **3. Eksplorasi:**
Saya menemunkan beberapa cara untuk mengambil parameter dari URL, seperti menggunakan `req.match_info().get("name")` untuk mengambil parameter `name` dari URL. Selain itu Actix menyediakan cara yang lebih rapih dan aman yang disebut Extractor. Untuk mengambil data dari path parameter, Anda dapat menggunakan `web::Path` extractor. Contohnya:

```rust
use actix_web::web;

#[get("/selamat-tinggal/{name}")]
// Actix akan otomatis mengambil 'name' dari path dan memasukkannya ke variabel `path`
async fn goodbye(path: web::Path<(String,)>) -> impl Responder {
    let name = &path.into_inner().0;
    format!("Selamat Tinggal {}, dan selamat jalan!", name)
}
```

Bayangkan URL yang diakses adalah: `/selamat-tinggal/Budi`

**1. `path: web::Path<(String,)>`**

* `web::Path` adalah sebuah **Extractor**. Tugasnya adalah "mengekstrak" atau "mencabut" bagian dinamis dari URL (yang kita tandai dengan `{}` di rute).
* Bagian `<(String,)>` memberitahu Actix: "Saya berharap ada satu bagian dinamis di URL, dan saya ingin Anda mengubahnya menjadi sebuah `String`."
* Jadi, setelah Actix melihat `/selamat-tinggal/Budi`, `web::Path` akan berhasil mengekstrak `"Budi"` dan membungkusnya. Hasilnya adalah sebuah *struct* `Path` yang di dalamnya berisi sebuah **Tuple**.

**Apa itu Tuple?**
Tuple adalah kumpulan nilai dengan tipe yang bisa berbeda-beda, dikelompokkan menjadi satu. Dalam kasus kita, `(String,)` adalah tuple yang hanya berisi satu elemen, yaitu sebuah `String`.
* Jika rutenya `/{id}/{category}`, maka extract-nya `web::Path<(u32, String)>`.

**2. `path.into_inner()`**

* `path` itu ibarat kado yang sudah dibungkus rapi oleh Actix. Isinya adalah data yang kita mau (yaitu tuple `("Budi",)`).
* Metode `.into_inner()` artinya "buka bungkusnya dan ambil isinya".
* Jadi, setelah `path.into_inner()` dijalankan, yang kita dapatkan adalah isinya saja, yaitu tuple: `("Budi",)`.

**3. `.0` (Bagian Paling Penting)**

* Sekarang kita punya tuple `("Budi",)`. Bagaimana cara mengambil nilai "Budi" dari dalamnya?
* Di Rust, kita mengakses elemen tuple menggunakan notasi titik (`.`) diikuti dengan indeksnya, yang dimulai dari **0**.
* Jadi, `.0` artinya "berikan saya elemen pertama (indeks ke-0) dari tuple ini".
* Hasil dari `("Budi",).0` adalah nilai `String` yaitu `"Budi"`.

**Kesimpulan:**

Jadi, pemahaman Anda sudah hampir benar. `name` tidak berisi *path* mulai dari indeks 0, melainkan `name` berisi **elemen ke-0** dari **hasil ekstraksi** `path` tersebut, yang sudah berbentuk tuple.

Secara visual:
`path` -> `Path(("Budi",))` -> `into_inner()` -> `("Budi",)` -> `.0` -> `"Budi"`

Keuntungannya adalah kode ini lebih aman. Jika seseorang mengakses `/selamat-tinggal/123` dan Anda mengharapkan angka (`u32`), Actix akan otomatis menolak permintaan itu sebelum masuk ke logika *handler* Anda. Tidak perlu `unwrap()` yang bisa menyebabkan *panic* (crash).

-----
## **📝 Notes**
Untuk penjelasan lebih detail kita masuk ke modul ke 2 mengenai "Routing & Ekstraksi Data Request"
