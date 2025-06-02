use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
}

#[derive(Serialize)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
}

#[derive(Deserialize)]
pub struct InfoPencaharian {
    pub q: String,
    pub kategori: Option<String>,
}

#[derive(Deserialize)]
pub struct ProductQuery {
    pub category: Option<String>,
    pub in_stock: Option<bool>,
}
