use actix_web::{Responder, get, post, web};

use crate::models::{CreateUser, InfoPencaharian, ProductQuery, User};

#[post("/users")]
pub async fn create_user(user_payload: web::Json<CreateUser>) -> impl Responder {
    // Di dunia nyata, di sini kita akan menyimpan data ke database.
    // Untuk sekarang, kita hanya simulasi saja.

    let new_user = User {
        id: 1337,
        name: user_payload.name.clone(),
        email: user_payload.email.clone(),
    };

    web::Json(new_user)
}

#[get("/products")]
pub async fn products(filter: web::Query<ProductQuery>) -> impl Responder {
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

#[get("/cari")]
pub async fn cari(info: web::Query<InfoPencaharian>) -> impl Responder {
    let query = &info.q;
    match &info.kategori {
        Some(kategori) => {
            format!("Anda mencari '{}' dalam kategori {}.", query, kategori)
        }
        None => {
            format!("Anda mencari '{}' tanpa kategori.", query)
        }
    }
}
