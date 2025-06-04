use actix_web::{App, HttpServer, get, web};
use std::env;
use std::sync::Mutex;

struct AppState {
    counter: Mutex<usize>,
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> String {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;
    format!("Counter: {}", counter)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    let app_data = web::Data::new(AppState {
        counter: Mutex::new(0),
    });

    let host = env::var("HOST").expect("HOST harus diset di file .env");
    let port_str = env::var("PORT").expect("PORT harus diset di file .env");
    let port = port_str.parse::<u16>().expect("PORT harus berupa angka");

    println!("🚀 Menjalankan server di http://{}:{}", host, port);

    HttpServer::new(move || App::new().app_data(app_data.clone()).service(index))
        .bind((host, port))?
        .run()
        .await
}
