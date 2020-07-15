use actix_web::{get, web, App, HttpServer, Responder};

// Constant throughput without extracting path information
#[get("/")]
async fn index() -> impl Responder {
    "without extracting..."
}

// Continuously falling throughput with extracting path information
#[get("/{id}")]
async fn id(_id: web::Path<u32>) -> impl Responder {
    "with extracting..."
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index).service(id))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
