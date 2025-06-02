## Modul 2: Routing & Penanganan Request!

- Topik: Mendefinisikan rute (routes), mengelompokkannya (scopes), dan cara mengambil data dari permintaan klien (request path, query parameters, dan body JSON).
- Best Practice: Menggunakan extractor bawaan Actix untuk kode yang bersih dan aman saat mengambil data.


-----

### Topik 1: Mengekstrak *Query Parameters*

Kita mulai dengan *Query Parameters*. Anda pasti sering melihat ini di URL, bagian yang ada setelah tanda tanya (`?`).

**Contoh:** `https://toko-online.com/cari?q=laptop&kategori=elektronik`

Di sini, `q=laptop` dan `kategori=elektronik` adalah *query parameters*. Gunanya untuk memfilter, mencari, atau menyortir data tanpa mengubah path URL utamanya.

Di Actix, cara paling elegan untuk mengambil data ini adalah dengan *extractor* **`web::Query`** dan bantuan dari *library* `serde`.

**Langkah 1: Tambahkan `serde` ke `Cargo.toml`**

`serde` adalah *library* super populer di dunia Rust untuk (de)serialisasi data, termasuk mengubah *query parameters* menjadi sebuah `struct`.

Buka `Cargo.toml` dan tambahkan `serde`:

```toml
[dependencies]
actix-web = "4"
serde = { version = "1.0", features = ["derive"] }
```

**Langkah 2: Buat `struct` untuk Menampung Parameter**

Kita perlu mendefinisikan sebuah `struct` yang merepresentasikan data yang kita harapkan dari URL.

```rust
use serde::Deserialize;

#[derive(Deserialize)]
pub struct InfoPencarian {
    q: String,
    kategori: Option<String>, // Kita buat 'kategori' opsional
}
```

  * `#[derive(Deserialize)]` secara ajaib memberikan `struct` kita kemampuan untuk dibuat dari data eksternal (seperti query string).
  * `Option<String>` adalah cara Rust untuk menangani nilai yang mungkin ada atau tidak. Jika URL tidak menyertakan `kategori`, nilainya akan menjadi `None`.

**Langkah 3: Gunakan di Handler**

Sekarang kita gunakan `struct` tersebut di dalam `handler` dengan *extractor* `web::Query`.

```rust
use actix_web::{get, web, Responder};
use serde::Deserialize;

// ... struct InfoPencarian di sini ...

#[get("/cari")]
// Actix akan otomatis mengambil query string dan memasukkannya ke struct kita
async fn cari(info: web::Query<InfoPencarian>) -> impl Responder {
    let query = &info.q;
    // Kita bisa cek apakah kategori ada atau tidak
    match &info.kategori {
        Some(kategori) => {
            format!("Anda mencari '{}' dalam kategori '{}'", query, kategori)
        }
        None => {
            format!("Anda mencari '{}' tanpa kategori", query)
        }
    }
}
```

Sekarang, jika Anda menjalankan server dan mengakses:

  * `http://127.0.0.1:3000/cari?q=rust&kategori=buku` -\> Actix akan otomatis mengisi `info.q` dan `info.kategori`.
  * `http://127.0.0.1:3000/cari?q=mobil` -\> `info.kategori` akan menjadi `None`, dan kode kita tetap berjalan tanpa error\!

-----

### **Materi & Dokumentasi**

  * **Extractor `Query`:** Dokumentasi resmi Actix untuk `web::Query`.
      * [Query Extractor - Actix Docs](https://www.google.com/search?q=https://actix.rs/docs/extractors%23query)
  * **Pengenalan `serde`:** Situs resmi `serde` untuk memahami cara kerjanya.
      * [Serde](https://serde.rs/)

-----

### **Tantangan Modul 2: Filter Produk**

**Tujuan:** Buat sebuah *endpoint* baru `/products` yang bisa memfilter produk berdasarkan kriteria opsional.

**Persyaratan:**

1.  Buat sebuah `struct` baru, misalnya `ProductQuery`.
2.  `struct` ini harus bisa menerima dua *query parameter* **opsional**:
    * `category` (bertipe `String`)
    * `in_stock` (bertipe `bool`)
3.  Buat sebuah `handler` baru untuk rute `GET /products` yang menggunakan `struct` tersebut.
4.  `handler` harus mengembalikan sebuah `String` yang menjelaskan kriteria pencarian.
    * **Jika diakses tanpa parameter (`/products`):** "Mencari semua produk."
    * **Jika diakses dengan `/products?category=elektronik`:** "Mencari produk dalam kategori: elektronik."
    * **Jika diakses dengan `/products?in_stock=true`:** "Mencari produk yang ada stok."
    * **Jika diakses dengan keduanya:** "Mencari produk dalam kategori: elektronik dan yang ada stok."

**Petunjuk:**
* Ingat, untuk field yang opsional, gunakan `Option<T>`, misalnya `Option<String>` atau `Option<bool>`.
* Anda perlu menggunakan `match` atau `if let` untuk memeriksa apakah nilai `Option` tersebut `Some(value)` atau `None`.

**Jawaban Saya:**
```rust
#[derive(Deserialize)]
struct ProductQuery {
    category: Option<String>,
    in_stock: Option<bool>,
}

#[get("/products")]
async fn products(filter: web::Query<ProductQuery>) -> impl Responder {
    match (&filter.category, filter.in_stock) {
        (Some(cat), Some(stock)) => {
            if stock {
                format!("Mencari produk dalam kategori: {} dan yang ada stok.", cat)
            } else {
                format!(
                    "Mencari produk dalam kategori: {} dan yang tidak ada stok.",
                    cat
                )
            }
        }
        (Some(cat), None) => {
            format!("Mencari produk dalam kategori: {}.", cat)
        }
        (None, Some(stock)) => {
            if stock {
                "Mencari produk yang ada stok.".to_string()
            } else {
                "Mencari produk yang tidak ada stok.".to_string()
            }
        }
        (None, None) => "Mencari semua produk.".to_string(),
    }
}
```

Dengan ini, kita telah menyelesaikan bagian pertama dari Modul 2 tentang *Query Parameters*.

Langkah berikutnya adalah mempelajari bagian yang paling penting dari API modern: **Menerima dan Mengirim Data dalam Format JSON**.

-----

### Topik 2: Bekerja dengan JSON (`POST` Request)

Sejauh ini kita baru menangani request `GET`. Sekarang, kita akan belajar menangani request `POST`, yang biasanya digunakan untuk **membuat data baru**. Data ini dikirim oleh klien (misalnya, browser atau aplikasi mobile) di dalam *body* request, paling sering dalam format JSON.

**Skenario kita:** Membuat *endpoint* `POST /users` untuk membuat pengguna baru.

**Langkah 1: Definisikan Struktur Data**

Sama seperti `web::Query`, kita perlu `struct` untuk merepresentasikan data JSON yang masuk dan keluar. Kita akan butuh dua `struct`: satu untuk data yang kita terima, dan satu lagi untuk data yang kita kirim kembali sebagai respons.

```rust
use serde::{Deserialize, Serialize};

// Struct untuk data yang MASUK (payload dari klien)
// Klien hanya mengirim nama dan email.
#[derive(Deserialize)]
struct CreateUser {
    name: String,
    email: String,
}

// Struct untuk data yang KELUAR (respons dari server)
// Server akan membuat ID dan mengirim kembali data lengkap.
#[derive(Serialize)]
struct User {
    id: u32,
    name: String,
    email: String,
}
```

  * `#[derive(Deserialize)]`: Memberi tahu `serde` cara mengubah JSON menjadi `struct CreateUser`.
  * `#[derive(Serialize)]`: Memberi tahu `serde` cara mengubah `struct User` menjadi JSON.

**Langkah 2: Buat Handler dengan Extractor `web::Json`**

Kita akan menggunakan *extractor* `web::Json` untuk secara otomatis mem-parsing body request JSON ke dalam `struct` kita.

```rust
use actix_web::{post, web, App, HttpServer, Responder};
// ... definisi struct CreateUser dan User di sini ...

#[post("/users")]
async fn create_user(user_payload: web::Json<CreateUser>) -> impl Responder {
    // Di dunia nyata, di sini kita akan menyimpan data ke database.
    // Untuk sekarang, kita hanya simulasi saja.

    println!("Membuat user baru: {}", user_payload.name);

    // Membuat data user baru untuk dikirim kembali sebagai respons
    let new_user = User {
        id: 1337, // ID ini biasanya dari database
        name: user_payload.name.clone(), // kita clone karena user_payload akan 'hilang'
        email: user_payload.email.clone(),
    };

    // Mengirim kembali data user baru sebagai JSON
    // Actix akan otomatis set Content-Type: application/json
    web::Json(new_user)
}
```

Perhatikan kita menggunakan `#[post("/users")]` untuk menandakan ini adalah handler untuk method `POST`.

**Langkah 3: Cara Menguji Endpoint `POST`**

Anda tidak bisa menguji ini hanya dengan mengetik URL di browser (karena itu adalah request `GET`). Anda perlu alat seperti **cURL** (di terminal) atau aplikasi GUI seperti **Postman** atau **Insomnia**.

Berikut contoh menggunakan `cURL`:

```bash
curl -X POST http://127.0.0.1:3000/users \
   -H "Content-Type: application/json" \
   -d '{"name": "Andi", "email": "andi@example.com"}'
```

Jika berhasil, server Anda akan merespons dengan:

```json
{"id":1337,"name":"Andi","email":"andi@example.com"}
```

-----

### **Materi & Dokumentasi**

  * **Extractor `Json`:** Dokumentasi resmi Actix untuk `web::Json`.
      * [JSON Extractor - Actix Docs](https://www.google.com/search?q=https://actix.rs/docs/extractors%23json)

-----

Ini adalah inti dari membangun sebuah API\! Kita menerima data, memprosesnya, dan mengembalikan data terstruktur.

-----

* `Deserialize`: Untuk data yang **masuk** (IN). Server menerima JSON dari luar dan mengubahnya menjadi `struct` agar bisa diproses oleh Rust.
* `Serialize`: Untuk data yang **keluar** (OUT). Server memiliki data dalam bentuk `struct` dan mengubahnya menjadi JSON untuk dikirim sebagai respons.


### Langkah Berikutnya: Merapikan "Restoran" Kita

Sekarang "restoran" kita sudah punya beberapa "koki" (`handler`) dan "menu" (`route`). Tapi semuanya masih berantakan di satu ruangan (`main.rs`). Jika restoran kita semakin besar, ini akan jadi kacau.

Saatnya kita belajar menjadi manajer yang baik dengan **merapikan struktur proyek dan mengelompokkan rute (`Scoped Routes`)**.

**Tujuan:** Memindahkan kode ke dalam file-file terpisah (modul) agar lebih terorganisir.

**Langkah 1: Buat File untuk Model & Handler**

Di dalam folder `src`, buat dua file baru:
* `models.rs`: Untuk menampung semua `struct` kita (`User`, `CreateUser`, `ProductQuery`).
* `handlers.rs`: Untuk menampung semua fungsi `handler` kita (`create_user`, `products`, dll).

**Langkah 2: Pindahkan Kode & Jadikan Publik**

1.  **Potong (`cut`)** semua definisi `struct` dari `main.rs` dan **tempel (`paste`)** ke dalam `src/models.rs`. Tambahkan kata kunci `pub` agar bisa diakses dari file lain.

    **src/models.rs**
    ```rust
    use serde::{Deserialize, Serialize};

    // 'pub' membuat struct ini bisa digunakan di file lain
    #[derive(Serialize)]
    pub struct User {
        pub id: u32,
        pub name: String,
        pub email: String,
    }

    #[derive(Deserialize)]
    pub struct CreateUser {
        pub name: String,
        pub email: String,
    }
    // ...tambahkan struct lainnya juga di sini...
    ```

2.  **Potong** semua fungsi `handler` dari `main.rs` dan **tempel** ke `src/handlers.rs`. Tambahkan juga `pub`.

    **src/handlers.rs**
    ```rust
    use actix_web::{get, post, web, Responder};
    // Kita butuh 'use' untuk mengakses model kita
    use crate::models::{CreateUser, User};

    // 'pub' membuat fungsi ini bisa digunakan di file lain
    #[post("/users")]
    pub async fn create_user(user_payload: web::Json<CreateUser>) -> impl Responder {
        let new_user = User {
            id: 1337,
            name: user_payload.name.clone(),
            email: user_payload.email.clone(),
        };
        web::Json(new_user)
    }
    // ...tambahkan handler lainnya juga di sini...
    ```

**Langkah 3: Gunakan Modul dan `web::scope` di `main.rs`**

Sekarang `main.rs` kita akan menjadi jauh lebih bersih. Kita akan "mengimpor" modul kita dan menggunakan `web::scope` untuk mengelompokkan semua rute yang berhubungan dengan `/users`.

**src/main.rs**
```rust
use actix_web::{web, App, HttpServer};

// Daftarkan file kita sebagai modul
mod handlers;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Menjalankan server di http://127.0.0.1:3000");

    HttpServer::new(|| {
        App::new()
            // Daftarkan semua service dari handler kita.
            // Anda bisa buat service untuk products, dll dengan cara yang sama.
            .service(handlers::create_user)
            // .service(handlers::products) ...dan seterusnya
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
```

**`web::scope` untuk pengelompokan (Cara yang lebih baik):**
Untuk rute yang lebih kompleks, Anda bisa mengelompokkannya. Misalnya, semua rute `/users` (seperti `GET /users`, `POST /users`, `GET /users/{id}`) bisa dikelompokkan. Ini adalah topik yang sedikit lebih maju, tapi intinya adalah `main.rs` Anda tetap bersih.


Ini mungkin terlihat seperti banyak langkah, tapi ini adalah pola yang akan sangat membantu Anda di proyek-proyek selanjutnya agar tetap terorganisir. Ini adalah praktik terbaik di dunia nyata.

-----
## **📝 Notes**
Selanjutnya, kita akan memasuki salah satu modul paling krusial untuk membangun aplikasi nyata:
Modul 3: Berbagi State & Manajemen Konfigurasi
