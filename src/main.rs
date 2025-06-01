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
