use actix_web::{
    App,
    HttpServer,
    web,
    HttpResponse
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .service(
                web::scope("/api")
                    .route("/upload", web::post().to(HttpResponse::Ok))
            )
    })
    .bind("0.0.0.0:8080")?
    .workers(5)
    .run()
    .await
}