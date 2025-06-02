use actix_web::{App, HttpServer};

mod handlers;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Menjalankan server di http://127.0.0.1:3000");

    HttpServer::new(|| {
        App::new()
            .service(handlers::cari)
            .service(handlers::products)
            .service(handlers::create_user)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
